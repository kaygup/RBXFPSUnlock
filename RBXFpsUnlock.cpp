#include <iostream>
#include <filesystem>
#include <fstream>

namespace fs = std::filesystem;

std::string getRobloxLoc() {
	char* buf = nullptr;
	size_t sz = 0;
	_dupenv_s(&buf, &sz, "APPDATA");

	fs::path approaming = buf;
	fs::path appdata = approaming.parent_path();

	for (auto& file : fs::directory_iterator(appdata.string() + "/Local/Roblox/Versions")) {
		if (fs::exists(file.path().string() + "/RobloxPlayerBeta.exe")) {
			return file.path().string();
		}
	}
	return "null";
}

void RemClientSettings() {
	std::string rbxdir = getRobloxLoc();
	if (fs::exists(rbxdir + "/ClientSettings")) {
		fs::remove_all(rbxdir + "/ClientSettings");
	}
}

void WriteNewRobloxFPS(int fps) {
	std::string rbxdir = getRobloxLoc();
	fs::create_directory(rbxdir + "/ClientSettings");
	std::ofstream csjson(rbxdir + "/ClientSettings/ClientAppSettings.json");
	csjson << "{\"DFIntTaskSchedulerTargetFps\": "+ std::to_string(fps) + ", \"FFlagGameBasicSettingsFramerateCap5\": false,\"FFlagTaskSchedulerLimitTargetFpsTo2402\": \"False\"}";
	csjson.close();

}

int main() {
	system("title RBXFPSUnlock");
	if (getRobloxLoc() != "null") {
		std::string yor;
		std::cout << "Use Unlocked FPS? (y, n) ";
		std::cin >> yor;
		if (yor == "y") {
			int targetfps;
			std::cout << "FPS: ";
			std::cin >> targetfps;
			WriteNewRobloxFPS(targetfps);
		}
		else {
			RemClientSettings();
		}
		return 0;
	}
	else {
		std::cout << "Roblox not found." << std::endl;
		system("pause >nul");
	}
}
