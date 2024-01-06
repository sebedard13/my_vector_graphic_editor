import { AfterViewInit, Component, EmbeddedViewRef } from "@angular/core"
import { EventsService } from "./events.service"
import { MoveCoordService } from "./functionality/move-coord"

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.scss"],
})
export class AppComponent {
    constructor(protected eventsService: EventsService) {}
}
