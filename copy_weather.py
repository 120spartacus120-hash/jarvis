import os
import shutil
import re

downloads_dir = r"C:\Users\Asus\Downloads"
target_dir = r"C:\Users\Asus\Desktop\EXE\Jarvis\jarvis-master\resources\sound\weather"

os.makedirs(target_dir, exist_ok=True)

files = [f for f in os.listdir(downloads_dir) if f.startswith("[Jarvis]") and f.endswith(".mp3")]

for f in files:
    src = os.path.join(downloads_dir, f)
    
    # Extract temperature
    m = re.search(r'\[Jarvis\]\+?(\d+)\s*г', f)
    if m:
        num = m.group(1)
        dst = os.path.join(target_dir, f"t_{num}.mp3")
        shutil.copy2(src, dst)
        print(f"Copied {f} to t_{num}.mp3")
        continue

    # Other files
    lower_f = f.lower()
    if "темпе" in lower_f:
        shutil.copy2(src, os.path.join(target_dir, "temperatura.mp3"))
        print(f"Copied {f} to temperatura.mp3")
    elif "ветра" in lower_f and "нет" in lower_f:
        shutil.copy2(src, os.path.join(target_dir, "veter_net.mp3"))
        print(f"Copied {f} to veter_net.mp3")
    elif "слабы" in lower_f:
        shutil.copy2(src, os.path.join(target_dir, "veter_slabiy.mp3"))
        print(f"Copied {f} to veter_slabiy.mp3")
    elif "умеренный" in lower_f:
        shutil.copy2(src, os.path.join(target_dir, "veter_umerenny.mp3"))
        print(f"Copied {f} to veter_umerenny.mp3")
    elif "сильн" in lower_f and "ветер" in lower_f:
        shutil.copy2(src, os.path.join(target_dir, "veter_silniy.mp3"))
        print(f"Copied {f} to veter_silniy.mp3")
    elif "без о" in lower_f and "(1)" not in lower_f:
        shutil.copy2(src, os.path.join(target_dir, "bez_osadkov.mp3"))
        print(f"Copied {f} to bez_osadkov.mp3")
    elif "дождь" in lower_f:
        shutil.copy2(src, os.path.join(target_dir, "dozhd.mp3"))
        print(f"Copied {f} to dozhd.mp3")
    else:
        print(f"Ignored {f}")

print("Done.")