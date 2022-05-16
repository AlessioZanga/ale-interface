#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! Rust binding for the [Arcade-Learning-Environment](https://github.com/mgbellemare/Arcade-Learning-Environment).

use autocxx::prelude::*;

include_cpp! {
    #include "ale_interface.hpp"
    #include "RomSettings.hpp"
    #include "Adventure.hpp"
    #include "AirRaid.hpp"
    #include "Alien.hpp"
    #include "Amidar.hpp"
    #include "Assault.hpp"
    #include "Asterix.hpp"
    #include "Asteroids.hpp"
    #include "Atlantis.hpp"
    #include "Atlantis2.hpp"
    #include "Backgammon.hpp"
    #include "BankHeist.hpp"
    #include "BasicMath.hpp"
    #include "BattleZone.hpp"
    #include "BeamRider.hpp"
    #include "Berzerk.hpp"
    #include "Blackjack.hpp"
    #include "Bowling.hpp"
    #include "Boxing.hpp"
    #include "Breakout.hpp"
    #include "Carnival.hpp"
    #include "Casino.hpp"
    #include "Centipede.hpp"
    #include "ChopperCommand.hpp"
    #include "CrazyClimber.hpp"
    #include "Crossbow.hpp"
    #include "DarkChambers.hpp"
    #include "Defender.hpp"
    #include "DemonAttack.hpp"
    #include "DonkeyKong.hpp"
    #include "DoubleDunk.hpp"
    #include "Earthworld.hpp"
    #include "ElevatorAction.hpp"
    #include "Enduro.hpp"
    #include "Entombed.hpp"
    #include "Et.hpp"
    #include "FishingDerby.hpp"
    #include "FlagCapture.hpp"
    #include "Freeway.hpp"
    #include "Frogger.hpp"
    #include "Frostbite.hpp"
    #include "Galaxian.hpp"
    #include "Gopher.hpp"
    #include "Gravitar.hpp"
    #include "Hangman.hpp"
    #include "HauntedHouse.hpp"
    #include "Hero.hpp"
    #include "HumanCannonball.hpp"
    #include "IceHockey.hpp"
    #include "JamesBond.hpp"
    #include "JourneyEscape.hpp"
    #include "Kaboom.hpp"
    #include "Kangaroo.hpp"
    #include "KeystoneKapers.hpp"
    #include "Kingkong.hpp"
    #include "Klax.hpp"
    #include "Koolaid.hpp"
    #include "Krull.hpp"
    #include "KungFuMaster.hpp"
    #include "LaserGates.hpp"
    #include "LostLuggage.hpp"
    #include "MarioBros.hpp"
    #include "MiniatureGolf.hpp"
    #include "MontezumaRevenge.hpp"
    #include "MrDo.hpp"
    #include "MsPacman.hpp"
    #include "NameThisGame.hpp"
    #include "Othello.hpp"
    #include "Pacman.hpp"
    #include "Phoenix.hpp"
    #include "Pitfall.hpp"
    #include "Pitfall2.hpp"
    #include "Pong.hpp"
    #include "Pooyan.hpp"
    #include "PrivateEye.hpp"
    #include "QBert.hpp"
    #include "RiverRaid.hpp"
    #include "RoadRunner.hpp"
    #include "RoboTank.hpp"
    #include "Seaquest.hpp"
    #include "SirLancelot.hpp"
    #include "Skiing.hpp"
    #include "Solaris.hpp"
    #include "SpaceInvaders.hpp"
    #include "SpaceWar.hpp"
    #include "StarGunner.hpp"
    #include "Superman.hpp"
    #include "Surround.hpp"
    #include "Tennis.hpp"
    #include "Tetris.hpp"
    #include "TicTacToe3d.hpp"
    #include "TimePilot.hpp"
    #include "Trondead.hpp"
    #include "Turmoil.hpp"
    #include "Tutankham.hpp"
    #include "UpNDown.hpp"
    #include "Venture.hpp"
    #include "VideoCheckers.hpp"
    #include "VideoChess.hpp"
    #include "VideoCube.hpp"
    #include "VideoPinball.hpp"
    #include "WizardOfWor.hpp"
    #include "WordZapper.hpp"
    #include "YarsRevenge.hpp"
    #include "Zaxxon.hpp"
    #include "utils.hpp"
    // Base structs of the ALE.
    generate!("ale::Action")
    generate!("ale::ALEInterface")
    generate!("ale::ALERAM")
    generate!("ale::ALEScreen")
    generate!("ale::ALEState")
    generate!("ale::RomSettings")
    // Supported game settings.
    generate!("ale::AdventureSettings")
    generate!("ale::AirRaidSettings")
    generate!("ale::AlienSettings")
    generate!("ale::AmidarSettings")
    generate!("ale::AssaultSettings")
    generate!("ale::AsterixSettings")
    generate!("ale::AsteroidsSettings")
    generate!("ale::AtlantisSettings")
    generate!("ale::Atlantis2Settings")
    generate!("ale::BackgammonSettings")
    generate!("ale::BankHeistSettings")
    generate!("ale::BasicMathSettings")
    generate!("ale::BattleZoneSettings")
    generate!("ale::BeamRiderSettings")
    generate!("ale::BerzerkSettings")
    generate!("ale::BlackjackSettings")
    generate!("ale::BowlingSettings")
    generate!("ale::BoxingSettings")
    generate!("ale::BreakoutSettings")
    generate!("ale::CarnivalSettings")
    generate!("ale::CasinoSettings")
    generate!("ale::CentipedeSettings")
    generate!("ale::ChopperCommandSettings")
    generate!("ale::CrazyClimberSettings")
    generate!("ale::CrossbowSettings")
    generate!("ale::DarkChambersSettings")
    generate!("ale::DefenderSettings")
    generate!("ale::DemonAttackSettings")
    generate!("ale::DonkeyKongSettings")
    generate!("ale::DoubleDunkSettings")
    generate!("ale::EarthworldSettings")
    generate!("ale::ElevatorActionSettings")
    generate!("ale::EnduroSettings")
    generate!("ale::EntombedSettings")
    generate!("ale::EtSettings")
    generate!("ale::FishingDerbySettings")
    generate!("ale::FlagCaptureSettings")
    generate!("ale::FreewaySettings")
    generate!("ale::FroggerSettings")
    generate!("ale::FrostbiteSettings")
    generate!("ale::GalaxianSettings")
    generate!("ale::GopherSettings")
    generate!("ale::GravitarSettings")
    generate!("ale::HangmanSettings")
    generate!("ale::HauntedHouseSettings")
    generate!("ale::HeroSettings")
    generate!("ale::HumanCannonballSettings")
    generate!("ale::IceHockeySettings")
    generate!("ale::JamesBondSettings")
    generate!("ale::JourneyEscapeSettings")
    generate!("ale::KaboomSettings")
    generate!("ale::KangarooSettings")
    generate!("ale::KeystoneKapersSettings")
    generate!("ale::KingkongSettings")
    generate!("ale::KlaxSettings")
    generate!("ale::KoolaidSettings")
    generate!("ale::KrullSettings")
    generate!("ale::KungFuMasterSettings")
    generate!("ale::LaserGatesSettings")
    generate!("ale::LostLuggageSettings")
    generate!("ale::MarioBrosSettings")
    generate!("ale::MiniatureGolfSettings")
    generate!("ale::MontezumaRevengeSettings")
    generate!("ale::MrDoSettings")
    generate!("ale::MsPacmanSettings")
    generate!("ale::NameThisGameSettings")
    generate!("ale::OthelloSettings")
    generate!("ale::PacmanSettings")
    generate!("ale::PhoenixSettings")
    generate!("ale::PitfallSettings")
    generate!("ale::Pitfall2Settings")
    generate!("ale::PongSettings")
    generate!("ale::PooyanSettings")
    generate!("ale::PrivateEyeSettings")
    generate!("ale::QBertSettings")
    generate!("ale::RiverRaidSettings")
    generate!("ale::RoadRunnerSettings")
    generate!("ale::RoboTankSettings")
    generate!("ale::SeaquestSettings")
    generate!("ale::SirLancelotSettings")
    generate!("ale::SkiingSettings")
    generate!("ale::SolarisSettings")
    generate!("ale::SpaceInvadersSettings")
    generate!("ale::SpaceWarSettings")
    generate!("ale::StarGunnerSettings")
    generate!("ale::SupermanSettings")
    generate!("ale::SurroundSettings")
    generate!("ale::TennisSettings")
    generate!("ale::TetrisSettings")
    generate!("ale::TicTacToe3dSettings")
    generate!("ale::TimePilotSettings")
    generate!("ale::TrondeadSettings")
    generate!("ale::TurmoilSettings")
    generate!("ale::TutankhamSettings")
    generate!("ale::UpNDownSettings")
    generate!("ale::VentureSettings")
    generate!("ale::VideoCheckersSettings")
    generate!("ale::VideoChessSettings")
    generate!("ale::VideoCubeSettings")
    generate!("ale::VideoPinballSettings")
    generate!("ale::WizardOfWorSettings")
    generate!("ale::WordZapperSettings")
    generate!("ale::YarsRevengeSettings")
    generate!("ale::ZaxxonSettings")
    // Utility functions.
    generate!("ale::action_to_string")
    generate!("utils::string_to_path")
}

mod action;
pub use crate::action::ALEAction;

mod interface;
pub use crate::interface::ALEInterface;
