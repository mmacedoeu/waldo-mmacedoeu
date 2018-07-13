from __future__ import print_function
from __future__ import division
# https://www.learnopencv.com/install-opencv3-on-ubuntu/
import cv2
import numpy as np
import argparse

path = "../assets/"
extension = ".jpg"
scene = "gobbling"
scene2 = "knights"
scene3 = "gold"
scene4 = "musketeers"
scene5 = "alibaba"
scene6 = "morning"
refFilename = path + scene6 + extension
quality = "1_90quality"
quality8 = "7_30quality"
quality9 = "8_20quality"
quality10 = "9_10quality"
imFilename = path + scene6 + "_selfie" + quality10 + extension

GOOD_MATCH_PERCENT = 0.15
FEATURES_DENSITY = 256
MIN_AREA = 256
MIN_DENSITY_FACTOR = FEATURES_DENSITY * MIN_AREA

# read images from disk returning selfie and scene images


def readImages(im1, im2):
    cim1 = cv2.imread(im1, cv2.IMREAD_COLOR)
    cim2 = cv2.imread(im2, cv2.IMREAD_COLOR)
    # Weak assumption the smaller image is the selfie and the bigger one the scene
    if cim1.size > cim2.size:
        return cim2, cim1
    else:
        return cim1, cim2

# pre processes the image, converting to green and scaling
# TODO fine tune the minimal selfie size where still is possible to detect good feature descriptions
def preImages(im1, im2):
    im1Gray = cv2.cvtColor(im1, cv2.COLOR_BGR2GRAY)
    im2Gray = cv2.cvtColor(im2, cv2.COLOR_BGR2GRAY)
    # rf = FEATURES_DENSITY / (int(im1Gray.size / MIN_AREA))
    # print("rf : ", rf)
    # width1 = int(im1.shape[1] * rf)
    # height1 = int(im1.shape[0] * rf)
    # dim1 = (width1, height1)
    # # decimated to max factor perfomance otimization
    # rim1 = cv2.resize(im1Gray, dim1,
    #                   interpolation=cv2.INTER_AREA)
    # width2 = int(im2.shape[1] * rf)
    # height2 = int(im2.shape[0] * rf)
    # dim2 = (width2, height2)
    # rim2 = cv2.resize(im2Gray, dim2,
    #                   interpolation=cv2.INTER_AREA)
    return im1Gray, im2Gray

# TODO verify if ORB features is faster


def alignImagesORB(im1, im2):
    orb1 = cv2.ORB_create(FEATURES_DENSITY)
    orb2 = cv2.ORB_create(FEATURES_DENSITY * int(round((im1.size / im2.size))))
    keypoints1, descriptors1 = orb1.detectAndCompute(im1, None)
    keypoints2, descriptors2 = orb2.detectAndCompute(im2, None)

    print("keypoints1 len : ", len(keypoints1))
    print("keypoints2 len : ", len(keypoints2))
    # Match features.
    matcher = cv2.DescriptorMatcher_create(
        cv2.DESCRIPTOR_MATCHER_BRUTEFORCE_HAMMING)
    matches = matcher.match(descriptors1, descriptors2, None)
    # Sort matches by score
    print("matches len : ", len(matches))
    matches.sort(key=lambda x: x.distance, reverse=False)
    # Remove not so good matches
    numGoodMatches = int(len(matches) * GOOD_MATCH_PERCENT)
    print("numGoodMatches : ", numGoodMatches)
    matches = matches[:numGoodMatches]

# detect features and compute descriptors in order to find matches from selfie in scene
def detectMatchesSURF(im1, im2):
    detector1 = cv2.xfeatures2d_SURF.create(hessianThreshold=FEATURES_DENSITY)
    keypoints1, descriptors1 = detector1.detectAndCompute(im1, None)
    keypoints2, descriptors2 = detector1.detectAndCompute(im2, None)
    print("keypoints1 len : ", len(keypoints1))
    print("keypoints2 len : ", len(keypoints2))

    matcher = cv2.DescriptorMatcher_create(cv2.DescriptorMatcher_FLANNBASED)
    knn_matches = matcher.knnMatch(descriptors1, descriptors2, 2)

    ratio_thresh = 0.8
    good_matches = []
    for m, n in knn_matches:
        if m.distance < ratio_thresh * n.distance:
            good_matches.append(m)

    return keypoints1, keypoints2, descriptors1, descriptors2, good_matches

# align the transalator transform to make selfie match scene and transform corners to localize selfie into scene


def alignImagesSURF(im1, im2):
    keypoints1, keypoints2, descriptors1, descriptors2, good_matches = detectMatchesSURF(
        im1, im2)

    # mininum 4 descriptors matching to find the object
    if len(good_matches) > 3:
        # -- Get the corners from the image_1 ( the object to be "detected" )
        obj_corners = np.empty((4, 1, 2), dtype=np.float32)
        obj_corners[0, 0, 0] = 0
        obj_corners[0, 0, 1] = 0
        obj_corners[1, 0, 0] = im1.shape[1]
        obj_corners[1, 0, 1] = 0
        obj_corners[2, 0, 0] = im1.shape[1]
        obj_corners[2, 0, 1] = im1.shape[0]
        obj_corners[3, 0, 0] = 0
        obj_corners[3, 0, 1] = im1.shape[0]

        # -- Localize the object
        obj = np.empty((len(good_matches), 2), dtype=np.float32)
        scene = np.empty((len(good_matches), 2), dtype=np.float32)
        for i in range(len(good_matches)):
            # -- Get the keypoints from the good matches
            obj[i, 0] = keypoints1[good_matches[i].queryIdx].pt[0]
            obj[i, 1] = keypoints1[good_matches[i].queryIdx].pt[1]
            scene[i, 0] = keypoints2[good_matches[i].trainIdx].pt[0]
            scene[i, 1] = keypoints2[good_matches[i].trainIdx].pt[1]

        H, _ = cv2.findHomography(obj, scene, cv2.RANSAC)
        scene_corners = cv2.perspectiveTransform(obj_corners, H)
        print("scene_corners : ", scene_corners)
        markDetected(scene_corners, cim2)
    else:
        print("object not found : ", len(good_matches))

# generate a new image with border lines highlighting the object found for DEBUG: purposes
def markDetected(scene_corners, im2):
    # -- Draw lines between the corners (the mapped object in the scene - image_2 )
    cv2.line(im2, (int(scene_corners[0, 0, 0]), int(scene_corners[0, 0, 1])),
             (int(scene_corners[1, 0, 0]), int(scene_corners[1, 0, 1])), (0, 255, 0), 4)
    cv2.line(im2, (int(scene_corners[1, 0, 0]), int(scene_corners[1, 0, 1])),
             (int(scene_corners[2, 0, 0]), int(scene_corners[2, 0, 1])), (0, 255, 0), 4)
    cv2.line(im2, (int(scene_corners[2, 0, 0]), int(scene_corners[2, 0, 1])),
             (int(scene_corners[3, 0, 0]), int(scene_corners[3, 0, 1])), (0, 255, 0), 4)
    cv2.line(im2, (int(scene_corners[3, 0, 0]), int(scene_corners[3, 0, 1])),
             (int(scene_corners[0, 0, 0]), int(scene_corners[0, 0, 1])), (0, 255, 0), 4)

    cv2.imwrite("matches.jpg", im2)


if __name__ == '__main__':

    parser = argparse.ArgumentParser(description='Prototype for finding if one image is cropped from another.')
    parser.add_argument('--input1', help='Path to input image 1.', default=imFilename)
    parser.add_argument('--input2', help='Path to input image 2.', default=refFilename)
    args = parser.parse_args()

    cim1, cim2 = readImages(args.input1, args.input2)
    dim1, dim2 = preImages(cim1, cim2)
    alignImagesSURF(dim1, dim2)
