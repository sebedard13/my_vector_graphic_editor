import { Injectable, Type } from "@angular/core";
import { Observable, Subject } from "rxjs";

export interface ModalComponent {
    closeModal(): Observable<unknown>;
}

@Injectable({
    providedIn: "root",
})
export class ModalService {
    private modalSubject = new Subject<Type<ModalComponent>>();

    showModal(component: Type<ModalComponent>) {
        this.modalSubject.next(component);
    }

    getModalObservable(): Observable<Type<ModalComponent>> {
        return this.modalSubject.asObservable();
    }
}
