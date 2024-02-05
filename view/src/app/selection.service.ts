import { Injectable } from "@angular/core";
import { Subject, filter } from "rxjs";
import { Rgba, Selected, SelectedLevel, set_color_of } from "wasm-vgc";
import { ScenesService } from "./scenes.service";
import { EventsService } from "./events.service";

@Injectable({
    providedIn: "root",
})
export class SelectionService {
    public selectedColor$: Subject<Rgba[]> = new Subject<Rgba[]>();

    public selection: Selected = new Selected();

    public selectionHasChanged: Subject<void> = new Subject<void>();

    constructor(
        private scenesService: ScenesService,
        eventsService: EventsService,
    ) {
        this.scenesService.currentSceneChange$.subscribe(() => {
            this.selection = new Selected();
            this.selectionHasChanged.next();
        });

        this.selectionHasChanged.asObservable().subscribe(() => {
            this.scenesService.currentSceneNow((scene) => {
                const selectedColors = this.selection.get_selected_colors(scene);
                this.selectedColor$.next(selectedColors);
            });
        });

        eventsService.mouseDown$.pipe(filter((event) => event.buttons == 1)).subscribe((event) => {
            this.scenesService.currentSceneNow((scene) => {
                if (event.shiftKey) {
                    const point = scene.get_project_mouse(event.offsetX, event.offsetY);
                    this.selection.add_selection(scene, point);
                    this.selectionHasChanged.next();
                } else {
                    const point = scene.get_project_mouse(event.offsetX, event.offsetY);
                    this.selection.change_selection(scene, point);
                    this.selectionHasChanged.next();
                }
            });
        });

        eventsService.mouseMove$.subscribe((event) => {
            this.scenesService.currentSceneNow((scene) => {
                const pt = scene.get_project_mouse(event.offsetX, event.offsetY);

                //selection
                this.selection.change_hover(scene, pt);
            });
        });

        eventsService.keydown$.pipe(filter((event) => event.key == "Escape")).subscribe(() => {
            this.selection.clear_to_level(SelectedLevel.None);
            this.selectionHasChanged.next();
        });
    }

    public set_color(color: Rgba) {
        this.scenesService.currentSceneNow((scene) => {
            set_color_of(this.selection, scene, color);
            this.selectionHasChanged.next();
        });
    }
}
