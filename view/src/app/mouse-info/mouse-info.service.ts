import { Injectable } from "@angular/core";
import { Observable, Subject, map, withLatestFrom } from "rxjs";
import { EventsService } from "../events.service";
import { ScenesService } from "../scenes.service";

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
            withLatestFrom(scenesService.currentScene$),
            map(([coords, scene]) => {
                return scene.get_project_mouse(coords.x, coords.y);
            }),
        );
    }
}
