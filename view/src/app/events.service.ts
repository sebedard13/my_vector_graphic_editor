import { Injectable } from "@angular/core";
import { Subject } from "rxjs";

@Injectable({
    providedIn: "root",
})
export class EventsService {
    private keyCodeSubject = new Subject<string>();
    public keyCode$ = this.keyCodeSubject.asObservable();

    public mouseMove = new Subject<MouseEvent>();
    public mouseMove$ = this.mouseMove.asObservable();

    public wheel = new Subject<WheelEvent>();
    public wheel$ = this.wheel.asObservable();

    constructor() {
        document.addEventListener("keydown", (event: KeyboardEvent) => {
            this.keyCodeSubject.next(event.key);
        });
    }
}
