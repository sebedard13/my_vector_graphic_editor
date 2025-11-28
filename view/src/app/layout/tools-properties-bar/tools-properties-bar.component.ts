import { ChangeDetectionStrategy, Component, computed, inject, signal } from "@angular/core";
import { ColorPickerComponent } from "../../utilities/color-picker/color-picker.component";
import { NumberInputComponent } from "src/app/utilities/number-input/number-input.component";
import { SelectionService } from "src/app/scene/selection.service";
import { ScenesService } from "src/app/scene/scenes.service";
import { filter, map } from "rxjs";
import { Rgba } from "src/app/utilities/client/common";
import { toSignal } from "@angular/core/rxjs-interop";

@Component({
    selector: "app-tools-properties-bar",
    imports: [ColorPickerComponent, NumberInputComponent],
    templateUrl: "./tools-properties-bar.component.html",
    styleUrl: "./tools-properties-bar.component.scss",
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ToolsPropertiesBarComponent {
    private readonly selectionService = inject(SelectionService);
    private readonly sceneService = inject(ScenesService);

    private lastSelectedStrokedSize = 0;

    private readonly selectedStrokeColors$ = this.selectionService.selectionHasChanged.pipe(
        map(() => {
            return this.sceneService.currentScene();
        }),
        filter((scene) => scene !== null),
        map((scene) => {
            return this.selectionService.selection.get_selected_stroke_colors(scene!.sceneClient);
        }),
    );
    private readonly selectedStrokeColors = toSignal(this.selectedStrokeColors$, {
        initialValue: [],
    });
    protected strokeColor = computed(() => {
        const value = this.selectedStrokeColors();
        return value.length === 1
            ? new Rgba(value[0].r, value[0].g, value[0].b, value[0].a)
            : undefined;
    });
    protected strokeColorInvalid = computed(() => {
        const value = this.selectedStrokeColors();
        return value.length > 1;
    });

    private readonly selectedStrokeSize = this.selectionService.selectionHasChanged.pipe(
        map(() => {
            return this.sceneService.currentScene();
        }),
        filter((scene) => scene !== null),
        map((scene) => {
            const stokes = this.selectionService.selection.get_selected_stroke_sizes(
                scene!.sceneClient,
            );
            if (stokes.length > 1) {
                return NaN;
            }

            if (stokes.length === 0) {
                return this.lastSelectedStrokedSize;
            }

            return stokes[0] * scene!.sceneClient.camera_get_base_scale().x;
        }),
    );
    protected strokeSize = toSignal(this.selectedStrokeSize, { initialValue: 0 });

    protected changeStrokeColor(color: Rgba): void {
        this.sceneService.currentSceneNow((scene) => {
            scene.sceneClient.set_stroke_color_of(this.selectionService.selection, color);
            this.selectionService.selectionHasChanged.next();
        });
    }

    protected changeStrokeSize(size: number): void {
        this.sceneService.currentSceneNow((scene) => {
            let basescale = scene.sceneClient.camera_get_base_scale();
            this.lastSelectedStrokedSize = size;
            size = size / basescale.x;
            scene.sceneClient.set_stroke_size_of(this.selectionService.selection, size);
            this.selectionService.selectionHasChanged.next();
        });
    }
}
