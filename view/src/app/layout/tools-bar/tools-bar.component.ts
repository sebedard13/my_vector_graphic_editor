import {
    AfterViewInit,
    ChangeDetectionStrategy,
    Component,
    DestroyRef,
    ElementRef,
    OnDestroy,
    inject,
} from "@angular/core";
import { Button, toolsbarSvgBtn } from "../../interface/buttons";
import { EventsService } from "../../scene/events.service";
import { takeUntilDestroyed } from "@angular/core/rxjs-interop";
import { filter } from "rxjs";
@Component({
    selector: "app-tools-bar",
    templateUrl: "./tools-bar.component.html",
    styleUrls: ["./tools-bar.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ToolsBarComponent implements AfterViewInit, OnDestroy {
    buttons: Button[] = toolsbarSvgBtn();
    activeButton!: Button;

    private readonly eventService = inject(EventsService);
    private readonly destroyRef = inject(DestroyRef);
    private readonly elementRef = inject(ElementRef);

    private readonly mapKeybinds: Map<string, number> = new Map([
        ["Digit1", 0],
        ["Digit2", 1],
        ["Digit3", 2],
        ["Digit4", 3],
        ["Digit5", 4],
        ["Digit6", 5],
        ["Digit7", 6],
        ["Digit8", 7],
        ["Digit9", 8],
        ["Digit0", 9],
    ]);

    ngAfterViewInit(): void {
        const startIndex = 0;
        this.activeButton = this.buttons[startIndex];
        this.elementRef.nativeElement.querySelector("#btn-id-" + startIndex).click();

        this.eventService.keydown$
            .pipe(
                takeUntilDestroyed(this.destroyRef),
                filter((event) => this.mapKeybinds.has(event.code)),
            )
            .subscribe((event) => {
                const index = this.mapKeybinds.get(event.code);
                if (index != undefined && index < this.buttons.length) {
                    this.elementRef.nativeElement.querySelector("#btn-id-" + index).click();
                }
            });
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
