// https://github.com/webb-tools/bulletproof-gadgets/tree/main/src/crypto_constants/data/poseidon

// Parameter for:
// exponentiation = 17
// width = 3
// full rounds = 8
// partial rounds = 33
// prime field =
// 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001

// Sage script command:
// sage generate_parameters_grain.sage 1 0 255 3 8 33
// 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
use crate::poseidon::sbox::PoseidonSbox;

pub const FULL_ROUNDS: u8 = 8;
pub const PARTIAL_ROUNDS: u8 = 57;
pub const WIDTH: u8 = 3;
pub const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(17);

use crate::utils::parse_vec;
use ark_ff::PrimeField;

pub fn get_rounds_poseidon_bls381_x17_3<F: PrimeField>() -> Vec<F> {
	parse_vec(ROUND_CONSTS.to_vec())
}
pub fn get_mds_poseidon_bls381_x17_3<F: PrimeField>() -> Vec<Vec<F>> {
	parse_matrix(MDS_ENTRIES.iter().map(|x| x.to_vec()).collect::<Vec<_>>())
}

pub fn get_full_rounds_poseidon_bls381_x17_3<F: PrimeField>() -> u8 {
	FULL_ROUNDS
}
pub fn get_partial_rounds_poseidon_bls381_x17_3<F: PrimeField>() -> u8 {
	PARTIAL_ROUNDS
}

pub fn get_width_poseidon_bls381_x17_3<F: PrimeField>() -> u8 {
	WIDTH
}

pub fn get_sbox_poseidon_bls381_x17_3<F: PrimeField>() -> PoseidonSbox {
	SBOX
}

use super::{PoseidonParameters, parse_matrix};
pub fn get_poseidon_bls381_x17_3<F: PrimeField>() -> PoseidonParameters<F> {
	let rounds = get_rounds_poseidon_bls381_x17_3();
	let mds = get_mds_poseidon_bls381_x17_3();
	PoseidonParameters::<F>::new(
	  rounds,
	  mds,
	  FULL_ROUNDS,
	  PARTIAL_ROUNDS,
	  WIDTH,
	  SBOX,
	)
  }


pub const ROUND_CONSTS: [&str; 123] = [
	"0x5d772b29f1d7b793e319457b3b07d48d21aeec2097aa56cd3e30f9af0bbc8d89",
	"0x201a8ace9938a4dccc38d35d5ca1576c7668b4694727d0a9e0b5f46ea5ee375d",
	"0x5d67a7b4e7af2fde75af9ef1ff2dcd3533611f43c38838eb8f7feb848afc78bb",
	"0x1870813d6f76123c9e8ebd0b488d53e430298ffe13ff4e1bb1682fe976059ad9",
	"0x2c821c4177b99f4577fe0b79163306aea6d13a7dc6e3e4dad75df6c01064c224",
	"0x2809d94c958a04ae9a990b2edd7068cc4ea52c84b0dc12fcceac167c8461cff9",
	"0x44955d7e59d9d096a1146f934b666d6272638b7e6435ef0d700f55eedd0fbd34",
	"0x49d161278155978d74a93f6ec706cfe710b9fee31217b5d31846b5d52d488dcb",
	"0x247f1a2f59f327e5e1f97695e5a1cdab102657e8679cf04d34c657bd11705fc5",
	"0x61902a4e734f0c4ba2c563e4539b4a663eb0116ffb196fa9de36cdd3e5ad2102",
	"0x378302692b4a1726e5ee3a26a0d4078ab44b3df36617731f83a0712dbba58e95",
	"0x06399c9ace112078cf013876ba22158d325ce5a4810aa5007c56341dc1032084",
	"0x506321d2a00f28c69d37c800d3dfbb493e9dd50ab2c3a1de90c36d333a51952c",
	"0x1d741c58942539e2a73f96f1f049fa8b8b0c1b21d1bebd69092b2c2a3ddab447",
	"0x452bfaedbcc3764779401bc0abdaa263f843a14842adc299d7e89321a7701834",
	"0x2039b972744b5f301c3bb96d15d58c66d9145be94c46c2c66bd86a3a3e39251e",
	"0x43e361fd2a833db5689292b4caf1d50e926600b9066bf3567e3cc1a0455dfea3",
	"0x6d778ba7e0eda990101c0f659aaa312def48cfa8898ce9fcd25404c144ad083d",
	"0x4ad66669c34c9c48619aff1960d60ff766b659bf08f0ec4edd6c7a12aaa8d821",
	"0x03075c36878b137024db9a081e8a33c294f376944669eb920e5401f118d5095a",
	"0x67fa41ff21b887cb2a1b76579281dc788bfc8968048d82f34617813874a5c0e2",
	"0x1ba5eeaa48834005b26c3d2160222566cfe159641fc7f677018f14c6272407fc",
	"0x576b72efe5775705d53cf011b87926f525ef33050e41ed6fc968840f57f442d6",
	"0x1940cd32d08c3b5788ae1893d1b66d8d58dcd04c49ed6d74112a59c80ac28b1f",
	"0x0cc6e9feedf57d43dae61bb534d3ce991d595472abc3b9b2dbaa6333e73748c6",
	"0x47e22cd739e867f0ba9bdd1209243160370294163b4ea8fbfef668b613e680ab",
	"0x11660762b6a1ad160bfbf7a737ac0167b06bac7ab9e97425cb69e7496a4f4034",
	"0x3c7536a816dec062472604ce4a7f1b95b40743bd03025afcccfe53b416d98831",
	"0x6dcca5f9d8ac6af2c4380e2eb3c30698a48fdc7e94028279cee7f7ca5b88c51b",
	"0x6e7b41929ef9f7c5b6d6557adfaa5328b20d1882586db8e3fb3f5233e4aa8f97",
	"0x1da2d7476c5ecd6d08218df30d20730fe8895d8f0cdbb635a589abf745dfeaed",
	"0x0792062e6869bce903ad4ea4adb617284ca3a50743475b10b946e1e87401149f",
	"0x51377b059d068a9d7b03b2d86045c8ddf396ef1d2d0e6064f9a53c263a844945",
	"0x3f0d13f5591ebf128504ed18478b82fbd31011e2b091e6ff7b6b2ec77c62f34f",
	"0x2cb9ede7aa9222c82330792c7b8acb0a7f9e54c3dbcdade6aeeff4be0a98a3f9",
	"0x0e56b5d5d051cc9fb6a552e176b607ec38cb95878e998d76aa5cbf2eda427bb4",
	"0x715f793ce2e8fee77430a30ac93209a58844f07aad6b9ce9a09a4dbcdbf79675",
	"0x3c35317c532f02a7d90798524c520877baf4789fd58ed35a8f92cc9b53ed4536",
	"0x3dfbccab23672f22310b97863ae2ae7def399d72449828c9868b3a05e280a91a",
	"0x22ba7c51db72f4c8636f248a3884600b5ea7fb1de0fe25983c9ec37af8f6b1e1",
	"0x0056d1c0564a91436cb7c93a382452dcc423325f1149074cb1bbfd14c5cea26b",
	"0x11990168d9f9c82614970d46a8619bce16ce56719f732738a0ca895dfec3ac63",
	"0x1d9e91fb65392b240cd7384454317c15283c2616a08a1609774dd8afcc8f7b87",
	"0x01bd923c8fcbbeeaa99c852a312e4ffa93a0e3491e94a341b912eb1049c47ba2",
	"0x46f9939145c053bd903a63de1ce9ef98928f420472a10b6472a0f6b6139ba620",
	"0x733c2546c12f32d3cda7df097318fc2d9eaeb77a497194003b655f5b72d47456",
	"0x50a743901d498a84e2cfbac32affb22a1c120d71afebcb5178653991d5497fa1",
	"0x296bebb1c3e6617ff5e8562661784f1c8a454c282db71efa81444fdecfcfd07b",
	"0x33cbef36d91e1df1418015eb22ff88ce94fba8f16703bfb1e3976a19842f5ef4",
	"0x492f250b2463e735d1e0867423ac73e146de10f030a9e30ea480c855697a0a08",
	"0x17fca1112858b0e9e187874ca8fbe96e30d3737d94a7442b05003b16f1692592",
	"0x1953add866dcb8ecad4869e4a306d553909ea9ad70c4cfb3f8550c4260fe4501",
	"0x0baaf5862854abf2dc803a6252f98423006ab4fa986ed7cf5c2a4dbc428e4beb",
	"0x4af71aace01a3dc3bc981eceb0422e407781bcd1392185d4502bd80db4b11c2d",
	"0x3b48b371bf6b8b8a0553b6b5023c85761dea42f5c4d300cc5267db07aa449f44",
	"0x45d43f82ba1cf0b055745432e9750828fabf39e24f39e870ebb9c656b35212d7",
	"0x431fc3e7ca0d01084cce1e34447e5de5c4c82581f318abc141751819059acd30",
	"0x0eccc9e2c36fa70aeab478035c06a621dad4362e56619e094912934a88ac3576",
	"0x06612b53133b0ef7790904ece018e5523506633a732acb8a98476ceed31404e3",
	"0x0f1494bebb916b7ab4082bfc6d47c7bc1cc1cf2199e6930d76f1f6e2bd2860bc",
	"0x50b31f09176465637b857b659343df3eb62d5f5eb7a1a2fc6c0ff5a2ea207a27",
	"0x4125090e1f9d716c920f32767b0f7b8ba047b7317e77de4c52237f57263da917",
	"0x3f6a1b715dff930d847cecc48cd1180eb9fe29b6d3e8750013ccd75d14878552",
	"0x67e098a1711e91f662d6bd4d0e1347fd96471ae628de49fcd71a683edc43fc87",
	"0x088051253577b086de932df084451b2b0c1904d0a169e5cf408bd7cba57a7c26",
	"0x3d1c8e7a1e36f24f121a5ac488bf6dc2b606fba40d09a8056c57a22fb8a67cbf",
	"0x1f453af47424f13d60ae974cb4bbac0941a4672aa4bc95472eb6b1d5805ece32",
	"0x09dc8eb7ec96061621a7b56f53c60e23379fff0e421e5e40b0f4457fda8905bb",
	"0x6edfc69d90684387b46b8fc7bc19b0f88abdce2f80d9681442eeda275beda310",
	"0x253ed42b4a972d05e974326b5bc12106996fc2092b81629fe444548fd230540c",
	"0x32a2264f8fbe68653e41a1967ef0938e262b6aac1f5c32a2dc4180a87c1b763b",
	"0x6c9a0a771efa001dd472fdbcf6304d5134f3baa97562c21e60bcb85f430c6caf",
	"0x0ce0377445e16ab2e07102287ee2f357636f7aabb9db7093bfcca3deb6372252",
	"0x3551b47f1007fe22ca965204b9068d73f6ad4e46e81bbd61a51e7bc4a159f21d",
	"0x2b75a1ac8dbe0f8a63d2916cd0fd56101d9a12bf207c58832cbf43864855a768",
	"0x6481f58b9f1afe48203f95887ec812f8efacd904c22f13adf0237ee6e377f80f",
	"0x51f4be64d9e9c27e9b5c66b99e46d256bdacee74402df3dea5fb9fc81134b9a9",
	"0x0306bf22d16953e1c7b54a06510145f2469aab2a3e3eb551d217d289611d0ac1",
	"0x06a237d9d633f20c7bf56c8f651add34118902b1ba17881835d9efa0b31908b6",
	"0x265d94ffb32eab2342a85c81ef1d18120d95fe3f8aa830555c8db11dad727263",
	"0x19a9f6a2aa1c9972bb973d082b2c283f93794e1319944044526846c61ddca141",
	"0x4e5b183e70bbe21cb0170d3bbf164952ca631dc194084f86f98fbc95e39ab7a3",
	"0x39920749c129d8ccfe986342730b7ce278205bd9602d7ffc18cc60d25617868e",
	"0x2c5de14a6f943d82cf9e5d6c0f097ebe91be70415570ca4cac5873a339a263dd",
	"0x5cb508d0399fc3185cebc1a12f58d7fa0045d32c712170fefe206f8dc60f96cc",
	"0x5e6d567222480dd53b93a9e73d66ac28c8a2755de751c7e9078168344ee0d5a2",
	"0x70c7e7d75f49ceaa428c5636937eeaa0ef34f93962f2b70099fd285655af4aad",
	"0x66513043aeccb85cafec14d6fc3e2f27c6d29d352b735864b9ebcc52670f3e2e",
	"0x5e26a0e4ee5f76b890a5de15d4c45fb17fdd2aa37e9e0867104ee9e786c49f10",
	"0x65bc3d1fb869b589f52c4e1bf5484ec480c25958397bc31429af197dbfeaa287",
	"0x38a317708f64626f1b64ac8ce149a1e5a08d36aba61e0e793819f78dfcda2526",
	"0x27f1cc5a19b6e6da66d2a48f715126ece4ab7256957f8fa4cdaa7944db3ddce9",
	"0x3cbe401ba6b3b617ab75e5c1e15d96ac8157c15d9736d0e5a2d3c2f40eb58391",
	"0x4e2f22786cef17113d082e1068d87843da64fc2fbb682425d24b01cf05c84883",
	"0x0734dee726c833ef5a7d222faf0411e27f062151108ee761fb9a99befb834774",
	"0x087cfbf13828f5c40e2765f068b100a01fd8899fda981eceb719efa677486d82",
	"0x39e16678e42fd8c89ea1eba46d5d9b8765de9b288e02c0a30e06350479417925",
	"0x2480cb74e843d4c6f99dce06595cdcf8344cde223d30cb76642f1421aaa7e9c3",
	"0x47139137faff4a90b0f14d2ab932ac5b6c26532f492939f2a4d4b4a0b04d9ff4",
	"0x1d81696f2ef9a91bdc490a4fa4e2225907be1a98c8a71113e7a726dcf04464f0",
	"0x5537d790a77400bd245bf05c4fbf1161383701e457dd568dcdb59201018d0e5c",
	"0x3331a6396b7bd45222920053345fbde2da5d7c29347e0d42346972405e8ff290",
	"0x217089a1e35c632aef695b8cab9c22f178516c6a5fbad80875a4f829c8f928a7",
	"0x29777687c8aca30c56b2d393efd12d8c9a3b731759003375f04cd151d4a6540e",
	"0x50b68476d35acc7031d1b0eda405f230d2fcf53c74c3ca62ff44747588a5e081",
	"0x0e22ce3c35764329214c2ca1458ee18f0df601f1713a274a9a57141793614eef",
	"0x63ef3fe6a75d27936720e20a2a38b5d7f792c865b4ce0d579163b90be12d2370",
	"0x48ccb2904453dc8108beeab4de49615a55db917a1ecfb0ce334875c7927492aa",
	"0x346e73bf565e07edc27067f48219ab9f1dc7107086c3f9788ad3c7a6ca03e0b5",
	"0x28df5e659cc51ca86ef139daf9fd652bc0f609d833bfe849ff3ac85d4d48cf2b",
	"0x54a8ad58b0b288c26597688cd2e8abd09f74955ae6b67d2ce553703e43246a83",
	"0x0b0aa366363e65d12539e1127958758bf9c00c0c5d539ff5bcc030dc5712aa77",
	"0x70d1985b3bd32ceef9e4e8ac27e2cf8171813cbf5313c65502e24e7ce1ecd81a",
	"0x4b89156b992ec0dbba578dfd096103ab7a9b5d0e4a72fba97363025c4a9c215b",
	"0x172348634ee8f5686c5c98993acae02e3503ba3c28635139d55e5944634946f3",
	"0x05d8c0bd1d6b44c20d729b16d8def43818defc73af1098788d6c3c857226c058",
	"0x2139a955deb8971ae1c356242418d2445e74f66a54b9d610b49da9261af9cdd8",
	"0x086d467cceb8adf78d9d8b3aec6d09b22b73464da18d8d644685812099062453",
	"0x6b29e022f1cb486ffc36e55524426fece480e67f701bd89c070c9fa08bd27c99",
	"0x5abd415ec94243ea0c2f3bec670ddaa4bb175c9af0ccec15ebe30bbffe344203",
	"0x2cdb1742e922442e43a278c567f9030db393d5b41d065ffaf128b7275179e063",
	"0x4f941f1e20771396d975295325bbcefa5ca08f68defa3ebf27971b4294330222",
	"0x3792834ae3bca5489d3fd4b1efc7ba8d31e440c8e612f2ecc1033603c9b87c92",
];
pub const MDS_ENTRIES: [[&str; 3]; 3] = [
	[
		"0x27aad1d6257dba5efd1f4bf6bee322e87ed9af61c9a326d63fe6dc9308965a4f",
		"0x4d1adb9b608117da79705b82c58ba43939d5657fdbe1b31a7bf10af81642e8a8",
		"0x4e475db098ccf68af42217e80ce6e7f1b6ce378d5bde84436984e51b3fcd3e75",
	],
	[
		"0x69601ef67b48f524bf34e31d6bde31d298a09f2162861f40a228d28b800a4655",
		"0x6bed58e1b31a50cf781ebfb88d2870a59c8c19070e7315895fd715962d14e3ed",
		"0x4f10d74b473c38559fa54220a330e67feecbfd659fb01c36f7f6ae6c174db30f",
	],
	[
		"0x0f72c96fcd12b35fde687616a43eb5c994177e03095d01412823d3cd7119a4c1",
		"0x0a581aa60aba4333b2507f9c6fc25eb6bda21197b94ae9b8735d387ddf6164db",
		"0x71c0e933ccd2a5cb22e04696843fa241c0f94ff6162abea876895575fcb92b4b",
	],
];
