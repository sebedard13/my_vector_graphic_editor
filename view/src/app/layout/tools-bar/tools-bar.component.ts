import { AfterViewInit, ChangeDetectionStrategy, Component, OnDestroy } from "@angular/core";
import { Button, toolsbarSvgBtn } from "../../interface/button";
@Component({
    selector: "app-tools-bar",
    templateUrl: "./tools-bar.component.html",
    styleUrls: ["./tools-bar.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ToolsBarComponent implements AfterViewInit, OnDestroy {
    buttons: Button[] = toolsbarSvgBtn();
    activeButton!: Button;

    ngAfterViewInit(): void {
        this.activeButton = this.buttons[0];
        this.activeButton.functionality?.activate();
    }

    onclick(button: Button) {
        this.activeButton.functionality?.desactivate();
        button.functionality?.activate();
        this.activeButton = button;
    }

    ngOnDestroy(): void {
        for (const button of this.buttons) {
            button.functionality?.desactivate();
        }
    }
}
