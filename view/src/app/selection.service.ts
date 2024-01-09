import { Injectable } from "@angular/core";
import { Subject, filter, withLatestFrom } from "rxjs";
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
        this.selectionHasChanged.asObservable().subscribe(() => {
            this.scenesService.currentScene$.subscribe((scene) => {
                const selectedColors = this.selection.get_selected_colors(scene);
                this.selectedColor$.next(selectedColors);
            });
        });

        eventsService.mouseDown$
            .pipe(
                filter((event) => event.buttons == 1),
                withLatestFrom(this.scenesService.currentScene$),
            )
            .subscribe(([event, canvasContent]) => {
                if (event.shiftKey) {
                    const point = canvasContent.get_project_mouse(event.offsetX, event.offsetY);
                    this.selection.add_selection(canvasContent, point);
                    this.selectionHasChanged.next();
                } else {
                    const point = canvasContent.get_project_mouse(event.offsetX, event.offsetY);
                    this.selection.change_selection(canvasContent, point);
                    this.selectionHasChanged.next();
                }
            });

        eventsService.mouseMove$
            .pipe(withLatestFrom(this.scenesService.currentScene$))
            .subscribe(([event, canvasContent]) => {
                const pt = canvasContent.get_project_mouse(event.offsetX, event.offsetY);

                //selection
                this.selection.change_hover(canvasContent, pt);
            });

        eventsService.keydown$.pipe(filter((event) => event.key == "Escape")).subscribe(() => {
            this.selection.clear_to_level(SelectedLevel.None);
            this.selectionHasChanged.next();
        });
    }

    public set_color(color: Rgba) {
        this.scenesService.currentScene$.subscribe((scene) => {
            set_color_of(this.selection, scene, color);
            this.selectionHasChanged.next();
        });
    }
}
