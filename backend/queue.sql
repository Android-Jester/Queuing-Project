-- MariaDB dump 10.19-11.1.2-MariaDB, for Linux (x86_64)
--
-- Host: localhost    Database: queue_database
-- ------------------------------------------------------
-- Server version	11.1.2-MariaDB

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `Clients`
--

DROP TABLE IF EXISTS `Clients`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Clients` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` text NOT NULL,
  `account_number` varchar(255) NOT NULL,
  `national_id` varchar(16) NOT NULL,
  `password` varchar(255) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `national_id` (`national_id`)
) ENGINE=InnoDB AUTO_INCREMENT=101 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Clients`
--

LOCK TABLES `Clients` WRITE;
/*!40000 ALTER TABLE `Clients` DISABLE KEYS */;
INSERT INTO `Clients` VALUES
(1,'Adam Smith','500194995935','GHA-498444017-3','P@ssw0rd1'),
(2,'Brian Jones','128754511521','GHA-866424680-7','Secret12'),
(3,'Charles Lee','513681672537','GHA-179725372-0','MyPa55word'),
(4,'David Davis','302495359900','GHA-948153136-0','SecurePass8'),
(5,'Ethan Johnson','783841790256','GHA-975834492-8','1234Abcd'),
(6,'Frank Brown','156143202922','GHA-163601744-9','StrongP@ss'),
(7,'George Wilson','927097962385','GHA-248684620-7','1qazXSW@'),
(8,'Harry Miller','964823708927','GHA-395972870-4','Pass1234'),
(9,'lan Taylor','203612973036','GHA-923745782-0','SecurePa55'),
(10,'Jack Clark','843369396008','GHA-293306692-9','P@ssw0rd!'),
(11,'Kevin Green','818347120044','GHA-778875512-7','Abcd1234'),
(12,'Liam White','793889604327','GHA-150427176-9','Pa$$word1'),
(13,'Mark Martin','604146237463','GHA-987273578-0','12345Abc'),
(14,'Noah Thompson','398856731818','GHA-269805204-8','StrongP@ssword'),
(15,'Ryan Moore','463535747859','GHA-297459334-8','P@ssword123'),
(16,'Sean Lewis','716994050324','GHA-911141678-4','Qwerty123'),
(17,'Tyler Hall','631009119167','GHA-692295445-8','Pa$$w0rd'),
(18,'Zachary Baker','820360343390','GHA-776658364-8','1234567a'),
(19,'Aaron Young','557256277492','GHA-630705413-7','SecurePa$$'),
(20,'Blake Allen','590790734153','GHA-433491281-7','P@ssw0rd'),
(21,'Cody King','655547925617','GHA-543079014-3','MySecret1'),
(22,'Dylan Evans ','997272692583','GHA-905745080-9','1q2w3e4r'),
(23,'Evan Scott. ','653436310149','GHA-225386412-0','Pa$$w0rd123'),
(24,'Gavin Wright','434239397688','GHA-636316692-6','12qwaszx'),
(25,'Hunter Hill','390081004791','GHA-747787386-6','Abc12345'),
(26,'Jake Adams','432118225285','GHA-712559240-1','P@ss123'),
(27,'Kyle Nelson','291001086101','GHA-670083206-2','MyP@ssword'),
(28,'Lure Carter','721289064754','GHA-617939896-2','1234qwer'),
(29,'Mason Mitchell','146941657552','GHA-897304857-7','1Password!'),
(30,'Nathan Collins','411168484138','GHA-934572791-3','SecurePa55word'),
(31,'Oliver Roberts','962883426853','GHA-135241866-0','P@55word'),
(32,'Patrick Walker','264854423988','GHA-982816672-4','1234abcd'),
(33,'Riley Phillips','165668137271','GHA-835159986-8','Qwerty12'),
(34,'Seth Wood','976903305771','GHA-490979741-0','P@$$w0rd'),
(35,'Troy Coopere ','511166753264','GHA-772967067-4','Strong123'),
(36,'Wyatt Parker','487677223582','GHA-796763271-1','Pa$$word!'),
(37,'Alex Edwards','725581759229','GHA-625034752-1','Abcdef12'),
(38,'Ben Murphy','415296257944','GHA-861768657-4','1234P@ss'),
(39,'Cole Sanders','919007150413','GHA-850272845-2','Qwerty12!'),
(40,'Drew Morgan','191496235137','GHA-417140671-5','P@55w0rd'),
(41,'Finn Rogers. ','777391629233','GHA-286347125-7','1Qwerty!'),
(42,'Grayson Kelly','597978456495','GHA-693327043-1','Pa$$word1234'),
(43,'Isaac Stewart','732924439705','GHA-163214720-3','Qwerty!23'),
(44,'Jesse Campbell','819007193198','GHA-451423894-3','P@$$w0rd1'),
(45,'Kai James','303419073733','GHA-727032506-9','MyP@ssword1'),
(46,'Leo Watson','945564101545','GHA-173756323-5','SecureP@ssword'),
(47,'Max Ward','720559584080','GHA-901858975-6','P@ssw0rd1234'),
(48,'Sam Cox','674481042763','GHA-505044271-8','1234qwer!'),
(49,'Alice Smith','677081742042','GHA-426101312-4','1qaz2wsx'),
(50,'Brenda Jones','971228650418','GHA-203028306-6','P@ssw0rd!1'),
(51,'Carol Lee','224878486442','GHA-584879944-0','Pa$$w0rd123!'),
(52,'Donna Davis','200999093663','GHA-208015447-8','Qwerty!12345'),
(53,'Emily Johnson','226738131127','GHA-492881891-5','Secur3P@ssword!'),
(54,'Fiona Brown','397428176042','GHA-747493105-9','Abcd12345!'),
(55,'Grace Wilson','508707619111','GHA-594110698-5','MyP@ssword!123'),
(56,'Hannah Miller','915827303312','GHA-650727099-8','1234P@ssword!'),
(57,'Iris Taylor','558192019969','GHA-166949959-4','StrongP@ssw0rd!'),
(58,'Julia Clark','673762269947','GHA-875836604-9','P@55word123'),
(59,'Kelly Green','143074359921','GHA-486581405-1','1qazXSW@2!'),
(60,'Laura White','514345482023','GHA-335421028-3','Pa$$w0rd!12345'),
(61,'Mia Martin','824293856878','GHA-556826886-6','SecurePa$$w0rd!'),
(62,'Nicole Thompson','655796782638','GHA-212084857-4','1Password!123'),
(63,'Olivia Williams','526550560645','GHA-144950834-7','MyP@ssword12'),
(64,'Paige Anderson ','167294465959','GHA-845912138-3','Abcdef!12345'),
(65,'Rachel Moore','829226528984','GHA-172176828-9','P@ssword!123'),
(66,'Sarah Lewis','802695485932','GHA-524569283-4','Qwerty!123456'),
(67,'Tara Hall','919502248710','GHA-295815929-2','StrongP@ssword!'),
(68,'Zoe Baker','997740138256','GHA-323989498-6','Pa$$w0rd12345'),
(69,'Amy Young','433601419033','GHA-468555571-9','1qazXSW@!'),
(70,'Brooke Allen','775899186318','GHA-511665945-8','SecurePa$$w0rd123'),
(71,'Chloe King','166613384955','GHA-628912310-6','Pa$$word!12345'),
(72,'Daisy Evans','453490443194','GHA-272339844-0','Qwerty!1234'),
(73,'Ella Scott','185694590309','GHA-822025033-6','P@ssw0rd!12345'),
(74,'Faith Wright','402310260264','GHA-460331254-8','MySecretP@ssword!'),
(75,'Gina Hill','629311526211','GHA-906664686-7','1q2w3e4r!123'),
(76,'Ivy Adams','747166665950','GHA-758804088-5','Pa$$w0rd1234'),
(77,'Jade Nelson','670825563338','GHA-518806479-5','Abcd123!456'),
(78,'Kayla Carter','377896472721','GHA-592178505-9','P@ssw0rd!12'),
(79,'Leah Mitchell','294684150537','GHA-792553111-2','MyP@ssword!12'),
(80,'Maya Collins','154360426682','GHA-168779340-9','1234qwer!123'),
(81,'Nora Roberts','499122868773','GHA-187658901-0','Qwerty!1234567'),
(82,'Ruby Walker','403331015680','GHA-444515897-0','P@$$w0rd!1234'),
(83,'Stella Phillips','373021581910','GHA-778141530-5','StrongP@ss!123'),
(84,'Tina Wood','148622184731','GHA-410611524-4','Pa$$word!1234'),
(85,'Wendy Cooper','928334609932','GHA-421203616-8','Abcdef!1234'),
(86,'Zoe Parker','788228322265','GHA-735043183-0','1234P@ssword!'),
(87,'Anna Edwards','803853906688','GHA-541585094-1','Qwerty!12345678'),
(88,'Bella Murphy','128458313664','GHA-914560055-0','P@55word!123'),
(89,'Claire Sanders','675077943375','GHA-694084023-3','1234P@ss!word'),
(90,'Daisy Morgan','667405598901','GHA-738933059-4','Qwerty!12345'),
(91,'Emma Rogers','717961071114','GHA-866155930-1','P@$$w0rd!12345'),
(92,'Grace Kelly','522755044207','GHA-511955315-6','1Qwerty!123'),
(93,'Isla Stewart','287781449844','GHA-756243901-7','Pa$$word!12345'),
(94,'Jessica Campbell','787641620735','GHA-616004860-0','Qwerty!123456789'),
(95,'Katie James','322829350613','GHA-901230323-3','P@$$w0rd!123456'),
(96,'Lily Watson','531426486368','GHA-256786939-5','MyP@ssword!1234'),
(97,'Mia Ward','635754333456','GHA-958603204-3','SecureP@ss!123'),
(98,'Olivia Cox','858605437725','GHA-557826314-7','P@ssw0rd!123456'),
(99,'Lazarus King','137366846618','GHA-662472534-5','1qaz2wsx!123'),
(100,'Emmanuel Frimpong','626482320971','GHA-770274610-7','P@ssw0rd!1234567');
/*!40000 ALTER TABLE `Clients` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Guests_Clients`
--

DROP TABLE IF EXISTS `Guests_Clients`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Guests_Clients` (
  `national_id` varchar(15) NOT NULL,
  `name` text NOT NULL,
  `transaction_detail` text NOT NULL,
  `telephone_num` text NOT NULL,
  PRIMARY KEY (`national_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Guests_Clients`
--

LOCK TABLES `Guests_Clients` WRITE;
/*!40000 ALTER TABLE `Guests_Clients` DISABLE KEYS */;
/*!40000 ALTER TABLE `Guests_Clients` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `MainQueue`
--

DROP TABLE IF EXISTS `MainQueue`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `MainQueue` (
  `national_id` varchar(16) NOT NULL,
  `position` int(11) NOT NULL,
  `name` text NOT NULL,
  `sub_queue_position` int(11) NOT NULL,
  `assigned_server` varchar(255) NOT NULL,
  `server_location` int(11) NOT NULL,
  `activity` varchar(255) NOT NULL,
  `time_duration` int(11) NOT NULL,
  `time_joined` timestamp NOT NULL,
  PRIMARY KEY (`national_id`),
  KEY `assigned_server` (`assigned_server`),
  CONSTRAINT `MainQueue_ibfk_1` FOREIGN KEY (`assigned_server`) REFERENCES `Servers` (`server_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `MainQueue`
--

LOCK TABLES `MainQueue` WRITE;
/*!40000 ALTER TABLE `MainQueue` DISABLE KEYS */;
/*!40000 ALTER TABLE `MainQueue` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Servers`
--

DROP TABLE IF EXISTS `Servers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Servers` (
  `server_id` varchar(255) NOT NULL,
  `station` int(11) NOT NULL,
  `service_time` int(11) NOT NULL,
  `password` varchar(255) NOT NULL,
  `active` tinyint(1) NOT NULL,
  PRIMARY KEY (`server_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Servers`
--

LOCK TABLES `Servers` WRITE;
/*!40000 ALTER TABLE `Servers` DISABLE KEYS */;
INSERT INTO `Servers` VALUES
('55734681',3,233,'P@55w0rd',0),
('75373417',0,292,'P@ssw0rd!123456',0),
('87365268',2,140,'1234P@ss',1),
('88742759',1,93,'Abcdef!12345',1);
/*!40000 ALTER TABLE `Servers` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Transactions`
--

DROP TABLE IF EXISTS `Transactions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Transactions` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `detail` text NOT NULL,
  `server_id` varchar(255) NOT NULL,
  `client_national_id` varchar(15) NOT NULL,
  `duration` float NOT NULL,
  `created_date` timestamp NOT NULL,
  PRIMARY KEY (`id`),
  KEY `server_id` (`server_id`),
  CONSTRAINT `Transactions_ibfk_1` FOREIGN KEY (`server_id`) REFERENCES `Servers` (`server_id`)
) ENGINE=InnoDB AUTO_INCREMENT=110 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Transactions`
--

LOCK TABLES `Transactions` WRITE;
/*!40000 ALTER TABLE `Transactions` DISABLE KEYS */;
INSERT INTO `Transactions` VALUES
(1,'Withdrawal','75373417','GHA-822025033-6',179,'2022-06-16 18:20:30'),
(2,'Payment of Fees','88742759','GHA-866155930-1',68,'2022-08-28 17:07:51'),
(3,'Deposit','88742759','GHA-835159986-8',184,'2022-09-14 22:31:19'),
(4,'Foreign Exchange','87365268','GHA-212084857-4',104,'2022-07-10 18:30:53'),
(5,'Withdrawal','75373417','GHA-905745080-9',161,'2022-08-05 19:07:36'),
(6,'Withdrawal','87365268','GHA-650727099-8',253,'2022-07-10 18:30:53'),
(7,'Deposit','87365268','GHA-770274610-7',73,'2022-07-18 16:10:15'),
(8,'Bill Payment','87365268','GHA-914560055-0',175,'2022-07-06 23:15:02'),
(9,'Withdrawal','55734681','GHA-901858975-6',248,'2022-09-06 16:15:10'),
(10,'Payment of Fees','88742759','GHA-433491281-7',150,'2022-07-26 23:14:32'),
(11,'Bill Payment','87365268','GHA-460331254-8',241,'2022-07-23 23:28:49'),
(12,'Foreign Exchange','55734681','GHA-208015447-8',102,'2022-07-17 21:32:00'),
(13,'Bill Payment','75373417','GHA-484810485-9',300,'2022-07-18 16:10:15'),
(14,'Foreign Exchange','75373417','GHA-948153136-0',251,'2022-09-14 22:31:19'),
(15,'Deposit','75373417','GHA-650727099-8',269,'2022-08-23 23:45:28'),
(16,'Bill Payment','88742759','GHA-541585094-1',241,'2022-06-17 16:53:18'),
(17,'Withdrawal','88742759','GHA-272339844-0',258,'2022-07-15 22:00:25'),
(18,'Foreign Exchange','75373417','GHA-901858975-6',108,'2022-08-23 23:45:28'),
(19,'Withdrawal','87365268','GHA-650727099-8',142,'2022-08-25 18:53:27'),
(20,'Deposit','75373417','GHA-662472534-5',156,'2022-07-02 17:00:50'),
(21,'Payment of Fees','55734681','GHA-987273578-0',94,'2022-08-08 19:05:11'),
(22,'Withdrawal','87365268','GHA-172176828-9',287,'2022-06-24 16:27:35'),
(23,'Withdrawal','87365268','GHA-756243901-7',175,'2022-06-25 23:21:02'),
(24,'Foreign Exchange','87365268','GHA-163601744-9',67,'2022-09-07 21:28:47'),
(25,'Foreign Exchange','55734681','GHA-617939896-2',104,'2022-08-10 18:51:56'),
(26,'Bill Payment','75373417','GHA-616004860-0',73,'2022-07-10 18:30:53'),
(27,'Withdrawal','75373417','GHA-592178505-9',92,'2022-07-17 21:32:00'),
(28,'Bill Payment','55734681','GHA-417140671-5',163,'2022-08-28 17:07:51'),
(29,'Deposit','55734681','GHA-594110698-5',144,'2022-06-19 20:09:47'),
(30,'Bill Payment','75373417','GHA-173756323-5',241,'2022-08-11 22:43:15'),
(31,'Deposit','88742759','GHA-323989498-6',93,'2022-09-06 16:15:10'),
(32,'Payment of Fees','75373417','GHA-293306692-9',233,'2022-08-09 20:04:33'),
(33,'Deposit','55734681','GHA-486581405-1',183,'2022-07-02 17:00:50'),
(34,'Deposit','87365268','GHA-747493105-9',247,'2022-08-04 22:09:38'),
(35,'Bill Payment','75373417','GHA-901230323-3',288,'2022-08-13 19:07:47'),
(36,'Foreign Exchange','75373417','GHA-778141530-5',203,'2022-06-28 20:12:16'),
(37,'Bill Payment','75373417','GHA-747787386-6',169,'2022-06-21 18:10:25'),
(38,'Deposit','75373417','GHA-166949959-4',285,'2022-07-02 17:00:50'),
(39,'Deposit','75373417','GHA-323989498-6',245,'2022-06-22 23:40:06'),
(40,'Withdrawal','75373417','GHA-850272845-2',300,'2022-09-06 16:15:10'),
(41,'Payment of Fees','75373417','GHA-758804088-5',215,'2022-07-22 21:26:37'),
(42,'Withdrawal','75373417','GHA-792553111-2',251,'2022-08-21 23:08:44'),
(43,'Bill Payment','55734681','GHA-293306692-9',179,'2022-08-13 19:07:47'),
(44,'Deposit','55734681','GHA-421203616-8',278,'2022-09-04 19:14:44'),
(45,'Deposit','75373417','GHA-421203616-8',191,'2022-07-10 18:30:53'),
(46,'Foreign Exchange','55734681','GHA-693327043-1',179,'2022-08-31 18:22:21'),
(47,'Foreign Exchange','88742759','GHA-505044271-8',168,'2022-07-16 18:02:37'),
(48,'Withdrawal','87365268','GHA-670083206-2',194,'2022-06-14 23:42:20'),
(49,'Withdrawal','55734681','GHA-778141530-5',71,'2022-06-29 17:11:42'),
(50,'Bill Payment','88742759','GHA-203028306-6',114,'2022-07-14 16:05:23'),
(51,'Payment of Fees','87365268','GHA-426101312-4',233,'2022-07-25 21:40:45'),
(52,'Payment of Fees','87365268','GHA-163601744-9',152,'2022-07-12 20:27:16'),
(53,'Payment of Fees','87365268','GHA-426101312-4',63,'2022-08-19 16:17:35'),
(54,'Foreign Exchange','88742759','GHA-187658901-0',156,'2022-07-31 23:04:00'),
(55,'Bill Payment','87365268','GHA-906664686-7',233,'2022-07-06 23:15:02'),
(56,'Payment of Fees','55734681','GHA-735043183-0',191,'2022-08-02 17:32:36'),
(57,'Payment of Fees','75373417','GHA-556826886-6',161,'2022-06-12 21:56:46'),
(58,'Bill Payment','87365268','GHA-987273578-0',75,'2022-07-18 16:10:15'),
(59,'Deposit','75373417','GHA-727032506-9',215,'2022-08-23 23:45:28'),
(60,'Bill Payment','55734681','GHA-468555571-9',67,'2022-08-17 23:20:14'),
(61,'Bill Payment','55734681','GHA-911141678-4',255,'2022-06-20 23:40:06'),
(62,'Bill Payment','88742759','GHA-168779340-9',191,'2022-09-06 16:15:10'),
(63,'Foreign Exchange','88742759','GHA-256786939-5',285,'2022-09-12 18:17:03'),
(64,'Deposit','75373417','GHA-323989498-6',252,'2022-07-27 17:50:07'),
(65,'Withdrawal','55734681','GHA-923745782-0',218,'2022-07-06 23:15:02'),
(66,'Withdrawal','55734681','GHA-662472534-5',169,'2022-07-15 22:00:25'),
(67,'Deposit','88742759','GHA-163214720-3',175,'2022-09-16 22:01:02'),
(68,'Withdrawal','88742759','GHA-958603204-3',142,'2022-07-19 19:17:20'),
(69,'Withdrawal','55734681','GHA-670083206-2',179,'2022-06-10 18:44:05'),
(70,'Deposit','55734681','GHA-692295445-8',69,'2022-07-02 17:00:50'),
(71,'Payment of Fees','88742759','GHA-144950834-7',66,'2022-09-10 22:20:59'),
(72,'Payment of Fees','55734681','GHA-295815929-2',175,'2022-06-22 23:40:06'),
(73,'Withdrawal','55734681','GHA-778875512-7',116,'2022-09-04 19:14:44'),
(74,'Payment of Fees','75373417','GHA-460331254-8',106,'2022-06-10 18:44:05'),
(75,'Deposit','87365268','GHA-911141678-4',282,'2022-07-21 16:07:13'),
(76,'Deposit','88742759','GHA-636316692-6',287,'2022-08-04 22:09:38'),
(77,'Withdrawal','88742759','GHA-727032506-9',182,'2022-09-02 19:31:41'),
(78,'Withdrawal','87365268','GHA-905745080-9',104,'2022-08-19 16:17:35'),
(79,'Foreign Exchange','88742759','GHA-168779340-9',106,'2022-08-18 16:25:51'),
(80,'Foreign Exchange','88742759','GHA-426101312-4',148,'2022-06-13 17:22:42'),
(81,'Payment of Fees','87365268','GHA-492881891-5',71,'2022-08-31 18:22:21'),
(82,'Payment of Fees','55734681','GHA-948153136-0',169,'2022-08-02 17:32:36'),
(83,'Foreign Exchange','87365268','GHA-293306692-9',226,'2022-06-21 18:10:25'),
(84,'Payment of Fees','88742759','GHA-747493105-9',243,'2022-08-30 23:48:09'),
(85,'Deposit','88742759','GHA-518806479-5',140,'2022-08-23 23:45:28'),
(86,'Foreign Exchange','87365268','GHA-738933059-4',169,'2022-08-21 23:08:44'),
(87,'Payment of Fees','88742759','GHA-662472534-5',283,'2022-07-09 23:13:23'),
(88,'Deposit','75373417','GHA-498444017-3',258,'2022-06-25 23:21:02'),
(89,'Foreign Exchange','55734681','GHA-778875512-7',198,'2022-07-07 23:37:40'),
(90,'Payment of Fees','87365268','GHA-269805204-8',287,'2022-09-08 22:57:04'),
(91,'Withdrawal','75373417','GHA-861768657-4',184,'2022-09-04 19:14:44'),
(92,'Foreign Exchange','88742759','GHA-486581405-1',141,'2022-06-28 20:12:16'),
(93,'Bill Payment','75373417','GHA-208015447-8',130,'2022-08-27 19:01:55'),
(94,'Deposit','87365268','GHA-778875512-7',191,'2022-08-31 18:22:21'),
(95,'Withdrawal','75373417','GHA-662472534-5',152,'2022-07-30 21:16:48'),
(96,'Payment of Fees','55734681','GHA-845912138-3',83,'2022-09-07 21:28:47'),
(97,'Foreign Exchange','87365268','GHA-505044271-8',169,'2022-07-14 16:05:23'),
(98,'Payment of Fees','55734681','GHA-163214720-3',148,'2022-07-12 20:27:16'),
(99,'Foreign Exchange','87365268','GHA-735043183-0',71,'2022-07-27 17:50:07'),
(100,'Foreign Exchange','87365268','GHA-135241866-0',73,'2022-08-13 19:07:47'),
(101,'Deposit','75373417','GHA-498444017-3',6,'2023-07-27 23:42:46'),
(102,'Deposit','75373417','GHA-498444017-3',7,'2023-07-27 23:43:29'),
(103,'Deposit','75373417','GHA-498444017-3',290,'2023-07-27 23:48:12'),
(104,'Deposit','75373417','GHA-498444017-3',290,'2023-07-27 23:48:12'),
(105,'Deposit','75373417','GHA-498444017-3',4,'2023-07-27 23:49:14'),
(106,'Deposit','75373417','GHA-498444017-3',14,'2023-07-27 23:50:23'),
(107,'Withdrawal','55734681','GHA-498444017-3',6,'2023-07-28 10:28:56'),
(108,'SchoolFees','55734681','GHA-150427176-9',26,'2023-07-28 10:43:07'),
(109,'Withdrawal','55734681','GHA-498444017-3',123,'2023-07-28 11:43:05');
/*!40000 ALTER TABLE `Transactions` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `__diesel_schema_migrations`
--

DROP TABLE IF EXISTS `__diesel_schema_migrations`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `__diesel_schema_migrations` (
  `version` varchar(50) NOT NULL,
  `run_on` timestamp NOT NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `__diesel_schema_migrations`
--

LOCK TABLES `__diesel_schema_migrations` WRITE;
/*!40000 ALTER TABLE `__diesel_schema_migrations` DISABLE KEYS */;
INSERT INTO `__diesel_schema_migrations` VALUES
('20230721192013','2023-07-27 20:35:25');
/*!40000 ALTER TABLE `__diesel_schema_migrations` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2023-08-26 20:44:44
