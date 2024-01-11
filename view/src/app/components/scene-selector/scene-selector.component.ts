import { ChangeDetectionStrategy, Component } from "@angular/core";
import { ScenesService } from "src/app/scenes.service";


@Component({
    selector: "app-scene-selector",
    templateUrl: "./scene-selector.component.html",
    styleUrl: "./scene-selector.component.scss",
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SceneSelectorComponent {
    constructor(protected scenesService: ScenesService) {}
}
