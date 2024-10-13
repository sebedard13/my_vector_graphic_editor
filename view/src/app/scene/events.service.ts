import { Injectable } from "@angular/core";
import { Subject } from "rxjs";

@Injectable({
    providedIn: "root",
})
export class EventsService {
    private keydown = new Subject<KeyboardEvent>();
    public keydown$ = this.keydown.asObservable();

    public mouseMove = new Subject<MouseEvent>();
    public mouseMove$ = this.mouseMove.asObservable();

    public mouseDown = new Subject<MouseEvent>();
    public mouseDown$ = this.mouseDown.asObservable();

    public wheel = new Subject<WheelEvent>();
    public wheel$ = this.wheel.asObservable();

    public mouseEnter = new Subject<MouseEvent>();
    public mouseEnter$ = this.mouseEnter.asObservable();

    public mouseLeave = new Subject<MouseEvent>();
    public mouseLeave$ = this.mouseLeave.asObservable();

    public mouseUp = new Subject<MouseEvent>();
    public mouseUp$ = this.mouseUp.asObservable();

    constructor() {
        document.addEventListener("keydown", (event: KeyboardEvent) => {
            if (event.target instanceof HTMLInputElement) {
                return;
            }
            this.keydown.next(event);
        });
    }
}
