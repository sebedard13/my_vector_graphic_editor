import { Injectable } from "@angular/core";
import { Functionality } from "./functionality";
import { EventsService } from "../scene/events.service";
import { Subscription } from "rxjs";
import { ScreenCoord, toggle_handle } from "wasm-vgc";
import { ScenesService } from "../scene/scenes.service";
import { SelectionService } from "../scene/selection.service";

@Injectable({
    providedIn: "root",
})
export class ToggleHandleService extends Functionality {
    private subscriptions: Subscription[] = [];

    constructor(
        private eventsService: EventsService,
        private sceneService: ScenesService,
        private selectionService: SelectionService,
    ) {
        super();
    }

    activate(): void {
        const addRemove = this.eventsService.mouseDown$.subscribe((event) => {
            this.sceneService.currentSceneNow((scene) => {
                if (event.buttons == 1) {
                    toggle_handle(
                        this.selectionService.selection,
                        scene.canvasContent,
                        new ScreenCoord(event.offsetX, event.offsetY),
                    );
                }
            });
        });
        this.subscriptions.push(addRemove);
    }

    desactivate(): void {
        this.subscriptions.forEach((subscription) => subscription.unsubscribe());
        this.subscriptions = [];
    }

    isActivated(): boolean {
        return this.subscriptions.length > 0;
    }
}
