import {
    AfterViewInit,
    ChangeDetectionStrategy,
    Component,
    EnvironmentInjector,
    OnDestroy,
    inject,
    runInInjectionContext,
} from "@angular/core";
import { Button } from "../../interface/Button";
import { toolsbarSvgBtn } from "src/app/interface/Button";

@Component({
    selector: "app-tools-bar",
    templateUrl: "./tools-bar.component.html",
    styleUrls: ["./tools-bar.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ToolsBarComponent implements AfterViewInit, OnDestroy {
    buttons: Button[] = toolsbarSvgBtn;
    activeButton!: Button;

    private injector = inject(EnvironmentInjector);

    ngAfterViewInit(): void {
        for (const button of this.buttons) {
            runInInjectionContext(this.injector, () => {
                button.functionality?.inject();
            });
        }

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
