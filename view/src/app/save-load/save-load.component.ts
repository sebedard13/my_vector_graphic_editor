import { ChangeDetectionStrategy, Component } from "@angular/core";
import { ScenesService } from "../scenes.service";

@Component({
    selector: "app-save-load",
    templateUrl: "./save-load.component.html",
    styleUrl: "./save-load.component.scss",
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SaveLoadComponent {

    constructor(protected scenesService: ScenesService){}
}
