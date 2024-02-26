import { Injectable } from "@angular/core";
import { Observable, Subject, map } from "rxjs";
import { EventsService } from "../scene/events.service";
import { ScenesService } from "../scene/scenes.service";
import { ScreenCoord } from "wasm-vgc";

@Injectable({
    providedIn: "root",
})
export class MouseInfoService {
    private mousePos = new Subject<{ x: number; y: number }>();

    public normalizedMousePos$: Observable<{ x: number; y: number }>;
    public mousePos$ = this.mousePos.asObservable();

    constructor(eventService: EventsService, scenesService: ScenesService) {
        eventService.mouseMove$.subscribe((event) => {
            this.mousePos.next({ x: event.offsetX, y: event.offsetY });
        });

        this.normalizedMousePos$ = this.mousePos$.pipe(
            map((coords) => {
                const scene = scenesService.currentScene();
                if (!scene) {
                    return { x: Infinity, y: Infinity };
                }

                const coord = scene.canvasContent.camera_project(
                    new ScreenCoord(coords.x, coords.y),
                );
                const rtn = { x: coord.x(), y: coord.y() };
                coord.free();
                return rtn;
            }),
        );
    }
}
