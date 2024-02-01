import { ChangeDetectionStrategy, Component } from "@angular/core";
import { ScenesService } from "src/app/scenes.service";
import { faXmark } from "@fortawesome/free-solid-svg-icons";

@Component({
    selector: "app-scene-selector",
    templateUrl: "./scene-selector.component.html",
    styleUrl: "./scene-selector.component.scss",
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SceneSelectorComponent {
    protected faXmark = faXmark;

    constructor(protected scenesService: ScenesService) {}
}
