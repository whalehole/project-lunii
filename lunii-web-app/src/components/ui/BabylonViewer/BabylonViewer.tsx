'use client'

import {Engine, Scene, Vector3, HemisphericLight, ImportMeshAsync, ArcRotateCamera, Database} from "@babylonjs/core";
import { registerBuiltInLoaders } from "@babylonjs/loaders/dynamic";
import { useEffect, useRef } from "react";
import styles from "./BabylonViewer.module.css";

export default function BabylonViewer() {
    const reactCanvas = useRef(null);

    useEffect(() => {
        registerBuiltInLoaders();

        const { current: canvas } = reactCanvas;

        if (!canvas) return;

        const createScene = () => {
            return new Scene(engine);
        }

        const engine = new Engine(canvas);
        engine.enableOfflineSupport = true;
        Database.IDBStorageEnabled = true;

        const scene = createScene();

        const camera = new ArcRotateCamera(
            "camera1",
            Math.PI / 2,
            Math.PI / 2,
            0.55,
            new Vector3(0, 1.28, 0),
        )
        camera.attachControl(canvas, true);
        camera.minZ = 0.01;

        const light = new HemisphericLight("light", new Vector3(0, 1, 0), scene);
        light.intensity = 1.4;

        ImportMeshAsync(
            "https://s3.localhost.localstack.cloud:4566/elfera-assets/3d_models/carlotta.v0.glb",
            scene,
        ).then();

        engine.runRenderLoop(() => {
            scene.render();
        })

        return () => {
            scene.dispose();
            engine.dispose();
        };

    }, []);

    return <canvas className={styles.viewerCanvas} ref={reactCanvas} />;
}