'use client'

import {Engine, Scene, Vector3, HemisphericLight, ImportMeshAsync, ArcRotateCamera} from "@babylonjs/core";
import { registerBuiltInLoaders } from "@babylonjs/loaders/dynamic";
import { useEffect, useRef } from "react";
import styles from "./BabylonViewer.module.css";

export default function BabylonViewer() {
    registerBuiltInLoaders();

    const reactCanvas = useRef(null);

    useEffect(() => {
        const { current: canvas } = reactCanvas;

        if (!canvas) return;

        const createScene = () => {
            return new Scene(engine);
        }

        const engine = new Engine(canvas);
        const scene = createScene();

        const camera = new ArcRotateCamera(
            "camera1",
            Math.PI / 2,
            Math.PI / 2,
            0.5,
            new Vector3(0, 1.2, 0),
        )
        camera.attachControl(canvas, true);
        camera.minZ = 0.01;

        const light = new HemisphericLight("light", new Vector3(0, 1, 0), scene);
        light.intensity = 0.7;

        ImportMeshAsync(
            "http://localhost:4566/elfera-assets/3d_models/carlotta.glb?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=test%2F20250630%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20250630T153009Z&X-Amz-Expires=3600&X-Amz-SignedHeaders=host&X-Amz-Signature=5b0f489b6fd0d80e55bac9e1c1f79611f4217326d04d59296a38f51c0dc55ad9",
            scene,
        ).then();

        engine.runRenderLoop(() => {
            scene.render();
        })

    }, []);

    return <canvas className={styles.viewerCanvas} ref={reactCanvas} />;
}