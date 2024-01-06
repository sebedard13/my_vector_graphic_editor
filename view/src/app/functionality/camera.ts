import { Subscription, filter, withLatestFrom } from "rxjs"
import { Functionality } from "./Functionality"
import { EventsService } from "../events.service"
import { ScenesService } from "../scenes.service"
import { inject } from "@angular/core"

export class CameraService extends Functionality {
    private subscriptions: Subscription[] = []
    private eventsService!: EventsService
    private sceneService!: ScenesService

    inject(): void {
        this.eventsService = inject(EventsService)
        this.sceneService = inject(ScenesService)
    }

    activate(): void {
        let zoomEvent = this.eventsService.wheel$
            .pipe(withLatestFrom(this.sceneService.currentScene$))
            .subscribe(([event, canvasContent]) => {
                canvasContent.zoom(
                    event.deltaY * -1,
                    event.offsetX,
                    event.offsetY,
                )
            })
        this.subscriptions.push(zoomEvent)

        let moveEvent = this.eventsService.mouseMove$
            .pipe(
                filter((event) => event.buttons == 4),
                withLatestFrom(this.sceneService.currentScene$),
            )
            .subscribe(([event, canvasContent]) => {
                canvasContent.pan_camera(event.movementX, event.movementY)
            })
        this.subscriptions.push(moveEvent)
    }

    desactivate(): void {
        this.subscriptions.forEach((subscription) => subscription.unsubscribe())
        this.subscriptions = []
    }

    isActivated(): boolean {
        return this.subscriptions.length > 0
    }
}
