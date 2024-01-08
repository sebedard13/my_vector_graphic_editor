import { ChangeDetectionStrategy, Component } from "@angular/core";

@Component({
    selector: "app-save-load",
    templateUrl: "./save-load.component.html",
    styleUrl: "./save-load.component.scss",
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SaveLoadComponent {}
