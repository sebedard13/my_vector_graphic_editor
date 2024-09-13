import { ChangeDetectionStrategy, Component, DestroyRef, inject } from "@angular/core";
import { ArrayDataSource } from "@angular/cdk/collections";
import { FlatTreeControl, CdkTreeModule } from "@angular/cdk/tree";
import {
    faChevronRight,
    faChevronDown,
    faEye,
    faEyeSlash,
    faLock,
    faLockOpen,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeModule } from "@fortawesome/angular-fontawesome";
import { CdkDragDrop, CdkDropList, CdkDrag, CdkDragPlaceholder } from "@angular/cdk/drag-drop";
import {
    BehaviorSubject,
    distinctUntilChanged,
    filter,
    map,
    shareReplay,
    switchMap,
    timer,
} from "rxjs";
import { ScenesService } from "src/app/scene/scenes.service";
import { takeUntilDestroyed } from "@angular/core/rxjs-interop";
import { TreeViewModel } from "wasm-client";
import { LayerThumbnailComponent } from "./layer-thumbnail/layer-thumbnail.component";

/** Flat node with expandable and level information */
export interface LocalTreeViewModel {
    expandable?: boolean;
    isExpanded?: boolean;
    lockEdit?: boolean;
    hideLayer?: boolean;
}

export type MergedTreeViewModel = TreeViewModel & LocalTreeViewModel;
@Component({
    selector: "app-scene-viewer",
    templateUrl: "./scene-viewer.component.html",
    styleUrl: "./scene-viewer.component.scss",
    changeDetection: ChangeDetectionStrategy.OnPush,
    standalone: true,
    imports: [
        CdkTreeModule,
        FontAwesomeModule,
        CdkDropList,
        CdkDrag,
        CdkDragPlaceholder,
        LayerThumbnailComponent,
    ],
})
export class SceneViewerComponent {
    protected Icon = {
        close: faChevronRight,
        open: faChevronDown,
        showLayer: faEye,
        hideLayer: faEyeSlash,
        lockEditLayer: faLock,
        canEditLayer: faLockOpen,
    };

    private treeData = new BehaviorSubject<MergedTreeViewModel[]>([]);

    protected treeControl = new FlatTreeControl<MergedTreeViewModel>(
        (node) => node.level,
        (node) => node.expandable ?? false,
    );

    protected dataSource = new ArrayDataSource(this.treeData.pipe(shareReplay(1)));

    private scenes = inject(ScenesService);

    protected noScene = this.scenes.currentScene$.pipe(map((scene) => scene === null));
    constructor() {
        timer(0, 1000)
            .pipe(
                switchMap(() => this.scenes.currentScene$),
                filter((scene) => scene !== null),
                takeUntilDestroyed(inject(DestroyRef)),
                map((scene) => scene.sceneClient.get_tree_view()),
                distinctUntilChanged((a, b) => a.length === b.length),
            )
            .subscribe((treeView) => {
                this.treeData.next(treeView);
            });

        // this.dataSource.connect().subscribe((data) => {
        //     console.log(data);
        // });
    }

    protected getParentNode(node: MergedTreeViewModel) {
        const nodeIndex = this.treeData.getValue().indexOf(node);

        for (let i = nodeIndex - 1; i >= 0; i--) {
            if (this.treeData.getValue()[i].level === node.level - 1) {
                return this.treeData.getValue()[i];
            }
        }

        return null;
    }

    protected shouldRender(node: MergedTreeViewModel) {
        let parent = this.getParentNode(node);
        while (parent) {
            if (!parent.isExpanded) {
                return false;
            }
            parent = this.getParentNode(parent);
        }
        return true;
    }

    protected drop(
        $event: CdkDragDrop<MergedTreeViewModel, MergedTreeViewModel, MergedTreeViewModel>,
    ) {
        const array = this.treeData.getValue();
        const id_current = array[$event.previousIndex].layer_id;
        const id_position = array[$event.currentIndex].layer_id;

        this.scenes.currentSceneNow((scene) => {
            scene.sceneClient.move_layer(id_current, id_position);
            const treeView = scene.sceneClient.get_tree_view();
            this.treeData.next(treeView);
        });

        if ($event.dropPoint.x - $event.previousContainer.element.nativeElement.offsetLeft < 20) {
            //level = 0;
        }
    }

    protected trackByFn(index: number, item: MergedTreeViewModel) {
        return item.layer_id;
    }

    protected toggleShowLayer(node: MergedTreeViewModel) {
        node.hideLayer = node.hideLayer == undefined ? true : !node.hideLayer;

        this.scenes.currentSceneNow((scene) => {
            if (node.hideLayer) {
                scene.sceneClient.hide_layer(node.layer_id);
            } else {
                scene.sceneClient.show_layer(node.layer_id);
            }
        });
    }
}
