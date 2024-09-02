import { AsyncPipe } from "@angular/common";
import { ChangeDetectionStrategy, Component, inject, input } from "@angular/core";
import { filter, map, Observable, switchMap, timer } from "rxjs";
import { ScenesService } from "src/app/scene/scenes.service";

@Component({
    selector: "app-layer-thumbnail",
    templateUrl: "./layer-thumbnail.component.html",
    styleUrl: "./layer-thumbnail.component.scss",
    changeDetection: ChangeDetectionStrategy.OnPush,
    standalone: true,
    imports: [AsyncPipe],
})
export class LayerThumbnailComponent {
    public layerId = input.required<number>();

    private scenes = inject(ScenesService);

    protected src: Observable<string>;

    constructor() {
        const delay = Math.floor(Math.random() * 1000);

        this.src = timer(delay, 2000).pipe(
            switchMap(() => this.scenes.currentScene$),
            filter((scene) => scene !== null),
            map((scene) => {
                return scene.sceneClient.image_layer(this.layerId(), 30, 30);
            }),
        );
    }
}
