import { Component } from "@angular/core";
import { EventsService } from "./events.service";
import { CameraService } from "./functionality/camera";

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.scss"],
})
export class AppComponent {
    constructor(protected eventsService: EventsService) {
        const camera = new CameraService();
        camera.inject();
        camera.activate();
    }
}
