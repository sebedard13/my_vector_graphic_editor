import {
    ReplaySubject,
    Subject,
    Subscription,
    filter,
    map,
    shareReplay,
    take,
    withLatestFrom,
} from "rxjs";
import { EventsService } from "../events.service";
import { ScenesService } from "../scenes.service";
import { Injectable, inject } from "@angular/core";
import { Functionality } from "./functionality";

@Injectable({
    providedIn: "root",
})
export class CameraService extends Functionality {
    private subscriptions: Subscription[] = [];
    private eventsService!: EventsService;
    private sceneService!: ScenesService;

    private zoomChange = new Subject<void>();
    private zoom = new ReplaySubject<number>(1);
    public zoom$ = this.zoom.asObservable();
    constructor() {
        super();

        this.eventsService = inject(EventsService);
        this.sceneService = inject(ScenesService);

        this.zoomChange
            .asObservable()
            .pipe(
                withLatestFrom(this.sceneService.currentScene$),
                map(([_, scene]) => scene.get_zoom()),
                shareReplay(1),
            )
            .subscribe((zoom) => {
                this.zoom.next(zoom);
            });

        this.sceneService.currentScene$.pipe(take(1)).subscribe((_) => {
            this.zoomChange.next();
        });
    }

    activate(): void {
        const zoomEvent = this.eventsService.wheel$
            .pipe(withLatestFrom(this.sceneService.currentScene$))
            .subscribe(([event, canvasContent]) => {
                canvasContent.zoom(event.deltaY * -1, event.offsetX, event.offsetY);
                this.zoomChange.next();
            });
        this.subscriptions.push(zoomEvent);

        const moveEvent = this.eventsService.mouseMove$
            .pipe(
                filter((event) => event.buttons == 4 || event.shiftKey && event.buttons == 1),
                withLatestFrom(this.sceneService.currentScene$),
            )
            .subscribe(([event, canvasContent]) => {
                canvasContent.pan_camera(event.movementX, event.movementY);
            });
        this.subscriptions.push(moveEvent);
    }

    desactivate(): void {
        this.subscriptions.forEach((subscription) => subscription.unsubscribe());
        this.subscriptions = [];
    }

    isActivated(): boolean {
        return this.subscriptions.length > 0;
    }
}
