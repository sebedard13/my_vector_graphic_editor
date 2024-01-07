import { Component } from "@angular/core";
import { EventsService } from "./events.service";
import { CameraService } from "./functionality/camera.service";

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.scss"],
})
export class AppComponent {
    constructor(
        protected eventsService: EventsService,
        protected cameraService: CameraService,
    ) {
        cameraService.activate();
    }
}
