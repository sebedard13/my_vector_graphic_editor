import { Injectable } from "@angular/core";
import { Functionality } from "./functionality";
import { EventsService } from "../events.service";
import { Subscription, withLatestFrom } from "rxjs";
import { draw_shape } from "wasm-vgc";
import { ScenesService } from "../scenes.service";
import { SelectionService } from "../selection.service";

@Injectable({
    providedIn: "root",
})
export class DrawShapeService extends Functionality {
    private subscriptions: Subscription[] = [];

    constructor(
        private eventsService: EventsService,
        private sceneService: ScenesService,
        private selectionService: SelectionService,
    ) {
        super();
    }

    activate(): void {
        const addRemove = this.eventsService.mouseDown$
            .pipe(withLatestFrom(this.sceneService.currentScene$))
            .subscribe(([event, canvasContent]) => {
                if (event.buttons == 1) {
                    draw_shape(
                        this.selectionService.selection,
                        canvasContent,
                        event.offsetX,
                        event.offsetY,
                    );
                }
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