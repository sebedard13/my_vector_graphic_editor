import { Injectable } from "@angular/core";
import { Observable, Subject } from "rxjs";
import { Rgba, Selected, set_color_of } from "wasm-vgc";
import { ScenesService } from "./scenes.service";

@Injectable({
    providedIn: "root",
})
export class SelectionService {
    public selectedColor$: Subject<Rgba[]> = new Subject<Rgba[]>();

    public selection: Selected = new Selected();

    public selectionHasChanged: Subject<void> = new Subject<void>();

    constructor(private scenesService: ScenesService) {
        this.selectionHasChanged.asObservable().subscribe(() => {
            this.scenesService.currentScene$.subscribe((scene) => {
                let selectedColors = this.selection.get_selected_colors(scene);
                this.selectedColor$.next(selectedColors);
            });
        });
    }

    public set_color(color: Rgba) {
        this.scenesService.currentScene$.subscribe((scene) => {
            set_color_of(this.selection, scene, color);
            this.selectionHasChanged.next();
        });
    }
}
