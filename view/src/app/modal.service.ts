import { Injectable, Type } from "@angular/core";
import { BehaviorSubject, Observable } from "rxjs";

export interface ModalComponent {
    closeModal$(): Observable<unknown>;
}

@Injectable({
    providedIn: "root",
})
export class ModalService {
    private modalSubject = new BehaviorSubject<Type<ModalComponent> | null>(null);

    showModal(component: Type<ModalComponent>) {
        this.modalSubject.next(component);
    }

    getModalObservable(): Observable<Type<ModalComponent> | null> {
        return this.modalSubject.asObservable();
    }

    public closeModal() {
        this.modalSubject.next(null);
    }
}
