# Notes

As of anchor 0.30, idl's need to be converted into the new format using

```shell
$> anchor idl convert <input> --out <output>
```

This requires updating the input idl file with a field `metadata.address` that is set to the address of the deployed program