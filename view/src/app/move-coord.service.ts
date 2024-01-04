import { Injectable } from "@angular/core"
import { EventsService } from "./events.service"
import { move_coords_of } from "wasm-vgc"
import { ScenesService } from "./scenes.service"
import { takeUntilDestroyed } from "@angular/core/rxjs-interop"
import { withLatestFrom } from "rxjs"
import { SelectionService } from "./selection.service"

@Injectable({
    providedIn: "root",
})
export class MoveCoordService {
    constructor(
        eventsService: EventsService,
        sceneService: ScenesService,
        selectionService: SelectionService,
    ) {
        eventsService.mouseMove$
            .pipe(
                takeUntilDestroyed(),
                withLatestFrom(sceneService.currentScene$),
            )
            .subscribe(([event, canvas]) => {
                if (event.buttons === 1) {
                    move_coords_of(
                        selectionService.selection,
                        canvas,
                        event.movementX,
                        event.movementY,
                    )
                }
            })
    }
}
