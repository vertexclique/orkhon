# -*- coding: utf-8 -*-

import numpy as np
import tensorflow as tf
import cv2

with tf.gfile.FastGFile("tests/protobuf/mobilenet_v2_1.4_224_frozen.pb", 'rb') as f:
    graph_def = tf.GraphDef()
    graph_def.ParseFromString(f.read())
    tf.import_graph_def(graph_def, name='')

writer = tf.summary.FileWriter("./graphstore/", tf.get_default_graph())
writer.close()

input_placeholder = tf.get_default_graph().get_tensor_by_name('input:0')
print(input_placeholder.shape)
bias_add = tf.get_default_graph().get_tensor_by_name('MobilenetV2/Logits/Conv2d_1c_1x1/BiasAdd:0')
print(bias_add.shape)
relu6 = tf.get_default_graph().get_tensor_by_name('MobilenetV2/Conv_1/Relu6:0')
print(relu6.shape)

config = tf.ConfigProto()
config.gpu_options.allow_growth = True
sess = tf.Session(config=config)


def image_read(imname):
    image = cv2.imread(imname)
    image = cv2.resize(image, (448, 448))
    image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB).astype(np.float32)
    image = (image / 255.0) * 2.0 - 1.0
    return image

def tf_model_hook(args):
    # image_test = image_read('tests/img/cat.jpg')
    image_test = np.zeros((448, 448, 3,))
    print(image_test.shape)
    image_test_valid = np.expand_dims(image_test, 0)
    print(image_test_valid.shape)
    bias_add_value = sess.run(bias_add, feed_dict={input_placeholder: image_test_valid})
    relu6_val = sess.run(relu6, feed_dict={input_placeholder: image_test_valid})
    print(relu6_val.shape)
    return relu6_val.shape
