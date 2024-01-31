import { ChangeDetectionStrategy, Component } from "@angular/core";
import { ScenesService } from "../scenes.service";
import { ModalService } from "../modal.service";
import { NewSceneComponent } from "../new-scene/new-scene.component";

@Component({
    selector: "app-menu-bar",
    templateUrl: "./menu-bar.component.html",
    styleUrl: "./menu-bar.component.scss",
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MenuBarComponent {
    constructor(
        protected scenesService: ScenesService,
        private modalService: ModalService,
    ) {}

    newScene() {
        this.modalService.showModal(NewSceneComponent);
    }
}
