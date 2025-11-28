import { ChangeDetectionStrategy, Component } from "@angular/core";
import { ScenesService } from "../../scene/scenes.service";
import { ModalService } from "../../modal.service";
import { NewSceneComponent } from "../../new-scene/new-scene.component";
import { RenderService } from "../../scene/render.service";

@Component({
    selector: "app-menu-bar",
    templateUrl: "./menu-bar.component.html",
    changeDetection: ChangeDetectionStrategy.OnPush,
    standalone: false
})
export class MenuBarComponent {
    constructor(
        protected scenesService: ScenesService,
        private modalService: ModalService,
        protected renderService: RenderService,
    ) {}

    newScene() {
        this.modalService.showModal(NewSceneComponent);
    }
}
