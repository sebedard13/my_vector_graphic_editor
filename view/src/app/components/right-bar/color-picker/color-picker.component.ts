import {
    AfterViewInit,
    ChangeDetectionStrategy,
    Component,
    DestroyRef,
    ElementRef,
    Host,
    HostListener,
    ViewChild,
    inject,
} from "@angular/core"
import { BehaviorSubject, fromEvent } from "rxjs"
import { takeUntilDestroyed } from "@angular/core/rxjs-interop"
import { SelectionService } from "src/app/selection.service"
import { Rgba } from "wasm-vgc"

@Component({
    selector: "app-color-picker",
    templateUrl: "./color-picker.component.html",
    styleUrls: ["./color-picker.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ColorPickerComponent implements AfterViewInit {
    private _destory = inject(DestroyRef)
    @ViewChild("colorInput") colorInput!: ElementRef<HTMLInputElement>

    private lastColor: string = "#FF0000"

    private colorValue = new BehaviorSubject<string>("#FF0000")
    private colorIsValid = new BehaviorSubject<boolean>(true)

    protected colorIsValid$ = this.colorIsValid.asObservable()
    protected colorValue$ = this.colorValue.asObservable()

    constructor(private selectionService: SelectionService) {
        selectionService.selectedColor$.subscribe((selected) => {
            if (selected.length == 0) {
                this.colorValue.next(this.lastColor)
                this.colorIsValid.next(true)
            } else if (selected.length == 1) {
                this.colorValue.next(selected[0].to_small_hex_string())

                this.colorIsValid.next(true)
            } else {
                this.colorValue.next("#000000")
                this.colorIsValid.next(false)
            }
        })
    }

    ngAfterViewInit(): void {
        fromEvent(this.colorInput.nativeElement, "input")
            .pipe(takeUntilDestroyed(this._destory))
            .subscribe((event) => {
                const target = event.target as HTMLInputElement
                const color = target.value

                this.selectionService.set_color(
                    Rgba.from_small_hex_string(color),
                )
            })
    }
}
