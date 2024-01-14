import { AfterViewInit, Component, Type, ViewChild, ViewContainerRef } from "@angular/core";
import { EventsService } from "./events.service";
import { CameraService } from "./functionality/camera.service";
import { ModalService } from "./modal.service";
import { NewSceneComponent } from "./new-scene/new-scene.component";

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.scss"],
})
export class AppComponent implements AfterViewInit {
    @ViewChild("modalContainer", { read: ViewContainerRef }) modalContainer!: ViewContainerRef;

    constructor(
        protected eventsService: EventsService,
        protected cameraService: CameraService,
        private modalService: ModalService,
    ) {
        cameraService.activate();
        this.modalService.getModalObservable().subscribe((data) => {
            this.loadModalComponent(data);
        });
    }

    ngAfterViewInit(): void {
        this.modalService.showModal(NewSceneComponent);
    }

    private loadModalComponent(component: Type<unknown>) {
        const currentComponent = this.modalContainer.createComponent(component);
        this.modalContainer.element.nativeElement.appendChild(
            currentComponent.location.nativeElement,
        );
    }
}
