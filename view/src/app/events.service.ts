import { AfterViewInit, Injectable } from '@angular/core';
import { Observable, Subject } from 'rxjs';

@Injectable({
    providedIn: 'root'
})
export class EventsService {

    private keyCodeSubject = new Subject<string>();
    public keyCode$ = this.keyCodeSubject.asObservable();

    constructor() {
        console.log("KeyboardService.ngAfterViewInit()");

        document.addEventListener('keydown', (event: KeyboardEvent) => {
            console.log(event);
        });


    }




}
