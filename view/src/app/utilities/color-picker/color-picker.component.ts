import {
    AfterViewInit,
    ChangeDetectionStrategy,
    Component,
    DestroyRef,
    ElementRef,
    ViewChild,
    effect,
    inject,
    input,
    output,
} from "@angular/core";
import { BehaviorSubject, fromEvent } from "rxjs";
import { takeUntilDestroyed } from "@angular/core/rxjs-interop";
import { Rgba } from "../client/common";

@Component({
    selector: "app-color-picker",
    templateUrl: "./color-picker.component.html",
    styleUrls: ["./color-picker.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ColorPickerComponent implements AfterViewInit {
    private _destory = inject(DestroyRef);
    @ViewChild("colorInput") colorInputElement!: ElementRef<HTMLInputElement>;
    @ViewChild("canvasInvalidColor")
    canvasInvalidColor!: ElementRef<HTMLCanvasElement>;

    public color = output<Rgba>();
    public colorInput = input<Rgba | undefined>();
    public invalidColor = input<boolean>(false);

    private lastColor: string = "#000000";

    private colorValue = new BehaviorSubject<string>("#000000");

    protected colorValue$ = this.colorValue.asObservable();

    constructor() {
        effect(() => {
            const selected = this.colorInput();

            if (selected == undefined) {
                this.colorValue.next(this.lastColor);
            } else if (selected) {
                const rgba = selected;
                this.colorValue.next(rgba.toCSSHex());
            }
        });
    }

    ngAfterViewInit(): void {
        fromEvent(this.colorInputElement.nativeElement, "input")
            .pipe(takeUntilDestroyed(this._destory))
            .subscribe((event) => {
                const target = event.target as HTMLInputElement;
                const color = target.value;

                this.lastColor = color;
                this.color.emit(Rgba.fromCSSHex(color));
            });

        const ctx = this.canvasInvalidColor.nativeElement.getContext("2d", {
            alpha: false,
        }) as CanvasRenderingContext2D;

        const width = this.canvasInvalidColor.nativeElement.width;

        ctx.fillStyle = "#FFFFFF";
        ctx.fillRect(0, 0, width, width);
        ctx.fillStyle = "#000000";
        ctx.beginPath();
        ctx.moveTo(0, 0);
        ctx.lineTo(0, width);
        ctx.lineTo(width, 0);
        ctx.closePath();
        ctx.fill();
    }
}
