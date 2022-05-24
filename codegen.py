
t = []
for i in range(0,16):
    v = f"grass_borders_{i}"
    t.append(v)
    print(f"""let {v}: Handle<Image> = asset_server.load("BaseTerrain/Grass/BorderDebug/GrassBorder0.png");""")
print("[" + ', '.join(t)+ "]")
