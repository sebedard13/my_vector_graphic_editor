import { Injectable, inject } from "@angular/core";
import { EventsService } from "../scene/events.service";
import { ScenesService } from "../scene/scenes.service";
import { Subscription, filter } from "rxjs";
import { SelectionService } from "../scene/selection.service";
import { Functionality } from "./functionality";
import { ScreenCoord, ScreenLength2d } from "../utilities/client/common";

@Injectable({
    providedIn: "root",
})
export class MoveCoordService extends Functionality {
    private subscriptions: Subscription[] = [];
    private eventsService!: EventsService;
    private scenesService!: ScenesService;
    private selectionService!: SelectionService;

    constructor() {
        super();
        this.eventsService = inject(EventsService);
        this.scenesService = inject(ScenesService);
        this.selectionService = inject(SelectionService);
    }

    activate(): void {
        if (this.subscriptions.length > 0) {
            console.warn("MoveCoordService already activated");
            return;
        }

        const movePoint = this.eventsService.mouseMove$.subscribe((event) => {
            this.scenesService.currentSceneNow((scene) => {
                if (event.buttons === 1) {
                    scene.canvasContent.move_coords_of(
                        this.selectionService.selection,
                        new ScreenLength2d(event.movementX, event.movementY),
                    );
                }
            });
        });

        const selecShape = this.eventsService.mouseDown$
            .pipe(filter((event) => event.buttons == 1))
            .subscribe((event) => {
                this.scenesService.currentSceneNow((scene) => {
                    if (event.shiftKey) {
                        const point = scene.canvasContent.camera_project(
                            new ScreenCoord(event.offsetX, event.offsetY),
                        );
                        this.selectionService.selection.add_selection(scene.canvasContent, point);
                        this.selectionService.selectionHasChanged.next();
                    } else {
                        const point = scene.canvasContent.camera_project(
                            new ScreenCoord(event.offsetX, event.offsetY),
                        );
                        this.selectionService.selection.change_selection(
                            scene.canvasContent,
                            point,
                        );
                        this.selectionService.selectionHasChanged.next();
                    }
                });
            });

        this.subscriptions.push(selecShape);
        this.subscriptions.push(movePoint);
    }

    desactivate(): void {
        this.subscriptions.forEach((subscription) => subscription.unsubscribe());
        this.subscriptions = [];
    }

    isActivated(): boolean {
        return this.subscriptions.length > 0;
    }
}
