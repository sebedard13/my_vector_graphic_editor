import { Injectable } from "@angular/core";
import { Subject, filter } from "rxjs";
import { Rgba, ScreenCoord, UserSelectionClient } from "wasm-client";
import { ScenesService } from "./scenes.service";
import { EventsService } from "./events.service";

@Injectable({
    providedIn: "root",
})
export class SelectionService {
    public selectedColor$: Subject<Rgba[]> = new Subject<Rgba[]>();

    public selection = new UserSelectionClient();

    public selectionHasChanged: Subject<void> = new Subject<void>();

    constructor(
        private scenesService: ScenesService,
        eventsService: EventsService,
    ) {
        this.scenesService.currentScene$.subscribe(() => {
            this.selection.free();
            this.selection = new UserSelectionClient();
            this.selectionHasChanged.next();
        });

        this.selectionHasChanged.asObservable().subscribe(() => {
            this.scenesService.currentSceneNow((scene) => {
                const selectedColors = this.selection.get_selected_colors(scene.sceneClient);
                this.selectedColor$.next(selectedColors);
            });
        });

        eventsService.mouseMove$.subscribe((event) => {
            this.scenesService.currentSceneNow((scene) => {
                const pt = scene.sceneClient.camera_project({ x: event.offsetX, y: event.offsetY } as ScreenCoord);

                //selection
                this.selection.set_mouse_position(pt);
                this.selection.change_hover(scene.sceneClient, pt);
            });
        });

        eventsService.mouseLeave$.subscribe(() => {
            this.selection.set_mouse_position(undefined);
        });

        eventsService.keydown$.pipe(filter((event) => event.key == "Escape")).subscribe(() => {
            this.selection.clear_to_level("None");
            this.selectionHasChanged.next();
        });
    }

    public set_color(color: Rgba) {
        this.scenesService.currentSceneNow((scene) => {
            scene.sceneClient.set_color_of(this.selection, color);
            this.selectionHasChanged.next();
        });
    }
}
