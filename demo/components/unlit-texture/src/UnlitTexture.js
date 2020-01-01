import { LitElement, html, css } from 'lit-element';
import {Scene,Vector3Data,FileType} from "../../../pkg/wtvr3d.js";

export class UnlitTexture extends LitElement {
    get styles() {
        return css`
        :host {
            position : relative;
            display : block;
            height : 100%;
            width : 100%;
        }
        canvas {
            height : 100%;
            width : 100%;
        }
        `;
    }
    firstUpdated() {
        this.init();
    }

    async getAssets(){
        let response = await fetch("../../../assets/meshes/test_monkey-0.wmesh");
        let mesh_data = new Uint8Array(await response.arrayBuffer());
        let response2 = await fetch("../../../assets/materials/test_uniform_color.wmaterial");
        let material_data = new Uint8Array(await response2.arrayBuffer());
        let response3 = await fetch("../../../assets/materials/test_without_override.wmatinstance");
        let mat_inst_data = new Uint8Array(await response3.arrayBuffer());
        return [mesh_data, material_data, mat_inst_data];
    }

    async init() {
        const [mesh, material, material_instance] = await this.getAssets();
        const canvas = this.shadowRoot.querySelector("canvas");
        const context = canvas.getContext("webgl");
        const scene = new Scene();
        let camera_id = scene.create_camera_entity(16/9,3.14/4,1,1000, new Vector3Data(0,4,10),new Vector3Data(0,0,0));
        scene.initialize(canvas, context, camera_id);
        let mesh_id = scene.register_asset(mesh,FileType.WMesh);
        let material_id = scene.register_asset(material,FileType.WMaterial);
        //let matinstance_id = scene.register_asset(material_instance,FileType.WMatInstance);
        //let mesh_entity_id = scene.create_mesh_entity(mesh_id,matinstance_id);
    }
    render() {
        return html`<canvas></canvas>`;
    }
}