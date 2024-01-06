import { Component } from "@angular/core"
import { EventsService } from "./events.service"
import { CameraService } from "./functionality/camera"
import { filter } from "rxjs"

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.scss"],
})
export class AppComponent {
    constructor(protected eventsService: EventsService) {
        let camera = new CameraService()
        camera.inject()
        camera.activate()
    }
}
