import { Injectable, inject } from "@angular/core";
import { EventsService } from "../scene/events.service";
import { ScreenLength2d, move_coords_of } from "wasm-vgc";
import { ScenesService } from "../scene/scenes.service";
import { Subscription } from "rxjs";
import { SelectionService } from "../scene/selection.service";
import { Functionality } from "./functionality";

@Injectable({
    providedIn: "root",
})
export class MoveCoordService extends Functionality {
    private subscription: Subscription | null = null;
    private eventsService!: EventsService;
    private sceneService!: ScenesService;
    private selectionService!: SelectionService;

    constructor() {
        super();
        this.eventsService = inject(EventsService);
        this.sceneService = inject(ScenesService);
        this.selectionService = inject(SelectionService);
    }

    activate(): void {
        if (this.subscription !== null) {
            console.warn("MoveCoordService already activated");
            return;
        }

        this.subscription = this.eventsService.mouseMove$.subscribe((event) => {
            this.sceneService.currentSceneNow((scene) => {
                if (event.buttons === 1) {
                    move_coords_of(
                        this.selectionService.selection,
                        scene.canvasContent,
                        new ScreenLength2d(event.movementX, event.movementY),
                    );
                }
            });
        });
    }

    desactivate(): void {
        this.subscription?.unsubscribe();
        this.subscription = null;
    }

    isActivated(): boolean {
        return this.subscription !== null;
    }
}
