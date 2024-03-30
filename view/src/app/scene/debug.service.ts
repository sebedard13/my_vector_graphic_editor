import { Injectable } from "@angular/core";
import { EventsService } from "./events.service";
import { ScenesService } from "./scenes.service";

@Injectable({
    providedIn: "root",
})
export class DebugService {
    constructor(
        eventsService: EventsService,
        private sceneService: ScenesService,
    ) {
        eventsService.keydown$.subscribe((event) => {
            if (event.key === "d" && event.ctrlKey) {
                this.sceneService.currentSceneNow((scene) => {
                    console.log(scene.canvasContent.debug_string());
                });
            }
        });
    }
}
