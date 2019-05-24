def model_hook(k):
    for key, value in k.items():
        print ("KEY: ", key)
        print ("VALUE: ", value)

    k.update({
        "vertex": "clique"
    })

    return k
