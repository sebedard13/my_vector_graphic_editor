import { Component, ComponentRef, Type, ViewChild, ViewContainerRef } from "@angular/core";
import { EventsService } from "./events.service";
import { CameraService } from "./functionality/camera.service";
import { ModalComponent, ModalService } from "./modal.service";
import { map } from "rxjs";

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.scss"],
})
export class AppComponent {
    @ViewChild("modalContainer", { read: ViewContainerRef }) modalContainer!: ViewContainerRef;
    currentComponent: ComponentRef<ModalComponent> | null = null;

    protected showModal$ = this.modalService.getModalObservable().pipe(
        map((data) => {
            if (data) {
                return true;
            }
            return false;
        }),
    );

    constructor(
        protected eventsService: EventsService,
        protected cameraService: CameraService,
        private modalService: ModalService,
    ) {
        cameraService.activate();
        this.modalService.getModalObservable().subscribe((data) => {
            if (data) {
                this.loadModalComponent(data);
            } else if(this.currentComponent) {
                this.removeModal();
            }
        });
    }

    private removeModal() {
        this.currentComponent!.destroy();
        this.currentComponent = null;
    }

    private loadModalComponent(component: Type<ModalComponent>) {
        this.currentComponent = this.modalContainer.createComponent(component);
        this.modalContainer.element.nativeElement.appendChild(
            this.currentComponent.location.nativeElement,
        );
        this.currentComponent.instance.closeModal$().subscribe(() => {
            this.modalService.closeModal();
        });
    }
}
