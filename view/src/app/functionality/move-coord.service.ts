import { Injectable, inject } from "@angular/core";
import { EventsService } from "../events.service";
import { move_coords_of } from "wasm-vgc";
import { ScenesService } from "../scenes.service";
import { Subscription } from "rxjs";
import { SelectionService } from "../selection.service";
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
        this.subscription = this.eventsService.mouseMove$.subscribe((event) => {
            this.sceneService.currentSceneNow((canvas) => {
                if (event.buttons === 1) {
                    move_coords_of(
                        this.selectionService.selection,
                        canvas,
                        event.movementX,
                        event.movementY,
                    );
                }
            });
        });
    }

    desactivate(): void {
        this.subscription?.unsubscribe();
    }

    isActivated(): boolean {
        return this.subscription !== null;
    }
}
