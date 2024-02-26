import { Injectable, Signal, WritableSignal, computed, signal } from "@angular/core";
import { Observable, Subject, map } from "rxjs";
import { EventsService } from "../scene/events.service";
import { ScenesService } from "../scene/scenes.service";
import { ScreenCoord, Coord } from "wasm-vgc";

type Point = { x: number; y: number };

@Injectable({
    providedIn: "root",
})
export class MouseInfoService {
    private static INVALID_COORD = { x: Infinity, y: Infinity };

    public mouseInCanvas = signal(false);

    public mousePosSignal: WritableSignal<Point> = signal({ x: 0, y: 0 });
    public mouseCanvasPosSignal: Signal<Point>;
    public mouseCanvasScreenPosSignal: Signal<Point>;

    constructor(eventService: EventsService, scenesService: ScenesService) {
        this.mouseCanvasPosSignal = computed(() => {
            const scene = scenesService.currentScene();
            if (!scene) {
                return MouseInfoService.INVALID_COORD;
            }

            const mousePos = this.mousePosSignal();

            const coord = scene.canvasContent.camera_project(
                new ScreenCoord(mousePos.x, mousePos.y),
            );
            const rtn = { x: coord.x(), y: coord.y() };
            coord.free();
            return rtn;
        });

        this.mouseCanvasScreenPosSignal = computed(() => {
            const scene = scenesService.currentScene();
            if (!scene) {
                return MouseInfoService.INVALID_COORD;
            }

            const mousePos = this.mouseCanvasPosSignal();
            const screenCoord = scene.canvasContent.camera_unproject_to_canvas(
                new Coord(mousePos.x, mousePos.y),
            );
            const rtn = { x: screenCoord.x(), y: screenCoord.y() };
            screenCoord.free();
            return rtn;
        });

        eventService.mouseMove$.subscribe((event) => {
            this.mousePosSignal.set({ x: event.offsetX, y: event.offsetY });
        });

        eventService.mouseEnter$.subscribe((event) => {
            this.mousePosSignal.set({ x: event.offsetX, y: event.offsetY });
            this.mouseInCanvas.set(true);
        });

        eventService.mouseLeave$.subscribe((_) => {
            this.mouseInCanvas.set(false);
        });
    }
}
