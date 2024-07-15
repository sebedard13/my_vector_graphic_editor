import { Injectable } from "@angular/core";
import { EventsService } from "./events.service";
import { ScenesService } from "./scenes.service";
import { environment } from "src/environments/environment";
import { set_logger } from "wasm-client";

@Injectable({
    providedIn: "root",
})
export class DebugService {
    constructor(
        eventsService: EventsService,
        private sceneService: ScenesService,
    ) {
        if (environment.logLevel){
            set_logger(environment.logLevel);
        }

        eventsService.keydown$.subscribe((event) => {
            if (event.key === "d" && event.ctrlKey) {
                this.sceneService.currentSceneNow((scene) => {
                    console.log(scene.canvasContent.debug_string());
                });
            }
        });
    }
}
