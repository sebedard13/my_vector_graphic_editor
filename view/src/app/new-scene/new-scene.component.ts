import { ChangeDetectionStrategy, Component, inject } from "@angular/core";
import { ModalComponent } from "../modal.service";
import { Observable, Subject } from "rxjs";
import { ScenesService } from "../scene/scenes.service";

@Component({
    selector: "app-new-scene",
    templateUrl: "./new-scene.component.html",
    styleUrl: "./new-scene.component.scss",
    changeDetection: ChangeDetectionStrategy.OnPush,
    standalone: false
})
export class NewSceneComponent implements ModalComponent {
    public private = new Subject<void>();

    protected width = 1280;
    protected height = 720;
    protected ratio = this.width / this.height;
    protected ratioFaction = smallestFraction(this.width, this.height);
    protected name = "New Scene";
    protected keepRatio = false;

    private scenesService = inject(ScenesService);

    protected closeModal() {
        this.private.next();
    }

    protected switchOrientation() {
        const tmp = this.width;
        this.width = this.height;
        this.height = tmp;
        this.ratio = this.width / this.height;
        this.ratioFaction = smallestFraction(this.width, this.height);
    }

    protected widthChange() {
        if (this.keepRatio) {
            this.height = this.width / this.ratio;
        } else {
            this.ratio = this.width / this.height;
            this.ratioFaction = smallestFraction(this.width, this.height);
        }
    }

    protected heightChange() {
        if (this.keepRatio) {
            this.width = this.height * this.ratio;
        } else {
            this.ratio = this.width / this.height;
            this.ratioFaction = smallestFraction(this.width, this.height);
        }
    }

    protected ratioChange() {
        this.keepRatio = false;
        this.height = this.width / this.ratio;
    }

    protected createScene() {
        this.scenesService.addNewScene(this.width, this.height, this.name);
        this.closeModal();
    }

    closeModal$(): Observable<unknown> {
        return this.private.asObservable();
    }
}

function smallestFraction(numerator: number, denominator: number): string {
    const divisor = gcd(numerator, denominator);
    const n = numerator / divisor;
    const d = denominator / divisor;
    const result = n + "/" + d;
    return result;
}

function gcd(numerator: number, denominator: number): number {
    if (denominator === 0) {
        return numerator;
    }

    return gcd(denominator, numerator % denominator);
}
