import * as THREE from 'three';
import {OrbitControls} from "three/addons/controls/OrbitControls.js";

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);

const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
renderer.setAnimationLoop(animate);
document.addEventListener('click', onPointerClick);
document.body.appendChild(renderer.domElement);

const controls = new OrbitControls(camera, renderer.domElement);
controls.listenToKeyEvents(window);
controls.enableDamping = true;
controls.dampingFactor = 0.05;
controls.screenSpacePanning = false;
controls.minDistance = 50;
controls.maxDistance = 200;

const pointer = new THREE.Vector2();
const rayon = new THREE.Raycaster();
const geometry = new THREE.BoxGeometry(1, 1, 1);
const material = new THREE.MeshBasicMaterial({color: 0xFFFFFF});
const star_vals = [];
let stars = {};

window.logIn = function () {
    document.querySelector('.header').classList.add('loggedIn');
    document.querySelector('.starmenu').classList.add('loggedIn');
};

window.showAddStar = function (val) {
    if (val) {
        document.querySelector('.addstarmenu').classList.add('loggedIn');
    } else {
        document.querySelector('.addstarmenu').classList.remove('loggedIn');
    }
}

let chooseStarCoords = false;

window.chooseCoords = function () {
    chooseStarCoords = true;
}

window.bvChanged = function () {
    const form = document.querySelector('form.addstarmenu');
    const bv = +form.querySelector('#addstarbv').value;
    form.querySelector('#addstarcolor').setAttribute('value', bv2rgb(bv));
    form.querySelector('#addstarcolor').value = bv2rgb(bv);
}

window.addStar = async function () {
    document.querySelector('.addstarmenu').classList.remove('loggedIn');
    const form = document.querySelector('form.addstarmenu');
    const ra = +form.querySelector('#addstarra').value;
    const de = +form.querySelector('#addstarde').value;
    const ly = +form.querySelector('#addstardist').value;
    const mn = +form.querySelector('#addstarmagnitude').value;
    const bv = +form.querySelector('#addstarbv').value;
    const r = await fetch(`http://localhost:8000/api/star`, {
        method: 'POST',
        body: JSON.stringify({
            right_ascension: ra,
            declination: de,
            light_years: ly,
            magnitude: mn,
            bv_value: bv,
        })
    });
    if (r.ok) {
        clickedStar = null;
        document.querySelector('.starmenu').classList.remove('hasStar');
        await getStars();
    }
}

const rgbToHex = (r, g, b) => '#' + [r, g, b].map(x => {
    const hex = x.toString(16)
    return hex.length === 1 ? '0' + hex : hex
}).join('')

function bv2rgb(bv) {
    let r, g, b, t;
    if (bv < -0.4) {
        bv = -0.4
    }
    if (bv > 2.0) {
        bv = 2.0
    }
    if (bv >= -0.40 && bv < 0.00) {
        t = (bv + 0.40) / (0.40)
        r = 0.61 + 0.11 * t + 0.1 * t * t
        g = 0.70 + 0.07 * t + 0.1 * t * t
        b = 1.0
    } else if (bv >= 0.00 && bv < 0.40) {
        t = (bv) / (0.40)
        r = 0.83 + (0.17 * t)
        g = 0.87 + (0.11 * t)
        b = 1.0
    } else if (bv >= 0.40 && bv < 1.60) {
        t = (bv - 0.40) / (1.60 - 0.40)
        r = 1.0
        g = 0.98 - 0.16 * t
    } else {
        t = (bv - 1.60) / (2.00 - 1.60)
        r = 1.0
        g = 0.82 - 0.5 * t * t
    }
    if (bv >= 0.40 && bv < 1.50) {
        t = (bv - 0.40) / (1.50 - 0.40)
        b = 1.00 - 0.47 * t + 0.1 * t * t
    } else if (bv >= 1.50 && bv < 1.951) {
        t = (bv - 1.50) / (1.94 - 1.50)
        b = 0.63 - 0.6 * t * t
    } else {
        b = 0.0
    }
    return rgbToHex(Math.round(r * 255.0), Math.round(g * 255.0), Math.round(b * 255.0))
}

window.deleteStar = async function() {
    if (clickedStar === null) return;
    const r = await fetch(`http://localhost:8000/api/star/${clickedStar.id}`, {method: 'DELETE'});
    if (r.ok) {
        clickedStar = null;
        document.querySelector('.starmenu').classList.remove('hasStar');
        await getStars();
    }
}

window.updateStar = async function() {
    if (clickedStar === null) return;
    const form = document.querySelector('form.starmenu');
    clickedStar.light_years = +form.querySelector('#stardist').value;
    clickedStar.magnitude = +form.querySelector('#starmagnitude').value;
    const r = await fetch(`http://localhost:8000/api/star`, {
        method: 'PUT',
        body: JSON.stringify(clickedStar)
    });
    if (r.ok) {
        clickedStar = null;
        document.querySelector('.starmenu').classList.remove('hasStar');
        await getStars();
    }
}

window.getStars = async function () {
    while (star_vals.length > 0) {
        scene.remove(star_vals.pop());
    }
    stars = {};
    const star_response = await fetch('http://localhost:8000/api/star');
    const star_datas = await star_response.json();
    for (const star_data of star_datas) {
        const r = 250;
        const lat = star_data.declination;
        const lon = star_data.right_ascension;
        const bv = star_data.bv_value;
        const star = new THREE.Mesh(geometry, material);

        const gamma = ( 90 - lat) * Math.PI / 180;
        const theta = (180 - lon) * Math.PI / 180;
        star.position.set(
            r * Math.sin(gamma) * Math.cos(theta),
            r * Math.cos(gamma),
            r * Math.sin(gamma) * Math.sin(theta)
        );

        star_vals.push(star);
        stars[star.uuid] = star_data;
        scene.add(star);
    }
};

let clickedStar;

function onPointerClick(event) {
    if (event.target !== renderer.domElement) return;
    pointer.x = (event.clientX / window.innerWidth) * 2 - 1;
    pointer.y = -(event.clientY / window.innerHeight) * 2 + 1;
    rayon.setFromCamera(pointer, camera);
    if (chooseStarCoords) {
        let intersectionPoint = new THREE.Vector3();
        rayon.ray.intersectSphere(new THREE.Sphere(new THREE.Vector3(), 250), intersectionPoint);
        if (intersectionPoint) {
            const latRads = Math.acos(intersectionPoint.y / 250);
            const lngRads = Math.atan2(intersectionPoint.z, intersectionPoint.x);
            const de = (Math.PI / 2 - latRads) * (180 / Math.PI);
            const ra = (Math.PI - lngRads) * (180 / Math.PI);
            console.log(intersectionPoint);
            const gamma = ( 90 - de) * Math.PI / 180;
            const theta = (180 - ra) * Math.PI / 180;
            console.log(new THREE.Vector3(
                250 * Math.sin(gamma) * Math.cos(theta),
                250 * Math.cos(gamma),
                250 * Math.sin(gamma) * Math.sin(theta)
            ));
            chooseStarCoords = false;
            const form = document.querySelector('form.addstarmenu');
            form.querySelector('#addstarra').setAttribute('value', ra.toString());
            form.querySelector('#addstarra').value = ra.toString();
            form.querySelector('#addstarde').setAttribute('value', de.toString());
            form.querySelector('#addstarde').value = de.toString();
        }
        return;
    }
    const intersects = rayon.intersectObjects(star_vals, false);
    if (intersects.length > 0) {
        document.querySelector('.starmenu').classList.add('hasStar');
        document.querySelector('.addstarmenu').classList.remove('loggedIn');
        clickedStar = stars[intersects[0].object.uuid];
        const form = document.querySelector('form.starmenu');
        form.querySelector('#starcolor').setAttribute('value', bv2rgb(clickedStar.bv_value));
        form.querySelector('#starcolor').value = bv2rgb(clickedStar.bv_value);
        form.querySelector('#stardist').setAttribute('value', clickedStar.light_years);
        form.querySelector('#stardist').value = clickedStar.light_years;
        form.querySelector('#starmagnitude').setAttribute('value', clickedStar.magnitude);
        form.querySelector('#starmagnitude').value = clickedStar.magnitude;
        form.querySelector('#starra').setAttribute('value', clickedStar.right_ascension);
        form.querySelector('#starra').value = clickedStar.right_ascension;
        form.querySelector('#starde').setAttribute('value', clickedStar.declination);
        form.querySelector('#starde').value = clickedStar.declination;
    } else {
        clickedStar = null;
        document.querySelector('.starmenu').classList.remove('hasStar');
    }
}

function animate() {
    controls.update();
    renderer.render(scene, camera);
}